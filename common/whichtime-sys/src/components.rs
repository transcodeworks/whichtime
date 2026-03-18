//! Option C: Array-based components with bitflags
//!
//! This replaces HashMap<Component, i32> with a fixed-size array and bitflags
//! for tracking which components are known vs implied.
//!
//! Benefits:
//! - Copy semantics (no heap allocation, no cloning cost)
//! - Cache-friendly (entire struct fits in ~64 bytes)
//! - Branch-free access patterns

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// Date/time components that can be parsed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Component {
    Year = 0,
    Month = 1,
    Day = 2,
    Weekday = 3,
    Hour = 4,
    Minute = 5,
    Second = 6,
    Millisecond = 7,
    Meridiem = 8,
    TimezoneOffset = 9,
}

impl Component {
    /// Total number of components
    pub const COUNT: usize = 10;

    /// Get the corresponding flag for this component
    #[inline]
    pub const fn flag(self) -> ComponentFlags {
        ComponentFlags::from_bits_truncate(1 << (self as u8))
    }

    /// Get component name as string
    pub const fn as_str(self) -> &'static str {
        match self {
            Component::Year => "year",
            Component::Month => "month",
            Component::Day => "day",
            Component::Weekday => "weekday",
            Component::Hour => "hour",
            Component::Minute => "minute",
            Component::Second => "second",
            Component::Millisecond => "millisecond",
            Component::Meridiem => "meridiem",
            Component::TimezoneOffset => "timezoneOffset",
        }
    }
}

bitflags! {
    /// Bitflags for tracking which components are set
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct ComponentFlags: u16 {
        const YEAR = 1 << 0;
        const MONTH = 1 << 1;
        const DAY = 1 << 2;
        const WEEKDAY = 1 << 3;
        const HOUR = 1 << 4;
        const MINUTE = 1 << 5;
        const SECOND = 1 << 6;
        const MILLISECOND = 1 << 7;
        const MERIDIEM = 1 << 8;
        const TIMEZONE_OFFSET = 1 << 9;
    }
}

/// Fast, stack-allocated date/time components
///
/// This is the optimized replacement for HashMap-based ParsedComponents.
/// Total size: 10 * 4 (values) + 2 (known) + 2 (implied) = 44 bytes
#[derive(Debug, Clone, Copy, Default)]
pub struct FastComponents {
    /// Component values indexed by Component enum
    values: [i32; Component::COUNT],
    /// Flags for known (certain) components
    known: ComponentFlags,
    /// Flags for implied components  
    implied: ComponentFlags,
}

impl FastComponents {
    /// Create new empty components
    #[inline]
    pub const fn new() -> Self {
        Self {
            values: [0; Component::COUNT],
            known: ComponentFlags::empty(),
            implied: ComponentFlags::empty(),
        }
    }

    /// Create components with default implied values from a reference date
    pub fn with_defaults(reference: &crate::ReferenceWithTimezone) -> Self {
        use chrono::Datelike;

        let date = reference.instant;
        let mut components = Self::new();

        // Set default implied values
        components.imply(Component::Year, date.year());
        components.imply(Component::Month, date.month() as i32);
        components.imply(Component::Day, date.day() as i32);
        components.imply(Component::Hour, 12);
        components.imply(Component::Minute, 0);
        components.imply(Component::Second, 0);
        components.imply(Component::Millisecond, 0);

        components
    }

    /// Check if a component is certain (known)
    #[inline]
    pub fn is_certain(&self, comp: Component) -> bool {
        self.known.contains(comp.flag())
    }

    /// Check if a component has any value (known or implied)
    #[inline]
    pub fn has(&self, comp: Component) -> bool {
        let flag = comp.flag();
        self.known.contains(flag) || self.implied.contains(flag)
    }

    /// Get a component value (known or implied)
    #[inline]
    pub fn get(&self, comp: Component) -> Option<i32> {
        if self.has(comp) {
            Some(self.values[comp as usize])
        } else {
            None
        }
    }

    /// Assign a value to a component (makes it certain/known)
    #[inline]
    pub fn assign(&mut self, comp: Component, value: i32) -> &mut Self {
        let idx = comp as usize;
        let flag = comp.flag();
        self.values[idx] = value;
        self.known.insert(flag);
        self.implied.remove(flag);
        self
    }

    /// Imply a value for a component (only if not already known)
    #[inline]
    pub fn imply(&mut self, comp: Component, value: i32) -> &mut Self {
        if !self.known.contains(comp.flag()) {
            let idx = comp as usize;
            self.values[idx] = value;
            self.implied.insert(comp.flag());
        }
        self
    }

    /// Delete a component (remove from both known and implied)
    #[inline]
    pub fn delete(&mut self, comp: Component) -> &mut Self {
        let flag = comp.flag();
        self.known.remove(flag);
        self.implied.remove(flag);
        self.values[comp as usize] = 0;
        self
    }

    /// Check if only date components are certain (no time)
    #[inline]
    pub fn is_only_date(&self) -> bool {
        !self.is_certain(Component::Hour)
            && !self.is_certain(Component::Minute)
            && !self.is_certain(Component::Second)
    }

    /// Check if only time components are certain (no date)
    #[inline]
    pub fn is_only_time(&self) -> bool {
        !self.is_certain(Component::Weekday)
            && !self.is_certain(Component::Day)
            && !self.is_certain(Component::Month)
            && !self.is_certain(Component::Year)
    }

    /// Check if only weekday is certain
    #[inline]
    pub fn is_only_weekday_component(&self) -> bool {
        self.is_certain(Component::Weekday)
            && !self.is_certain(Component::Day)
            && !self.is_certain(Component::Month)
    }

    /// Check if date has unknown year
    #[inline]
    pub fn is_date_with_unknown_year(&self) -> bool {
        self.is_certain(Component::Month) && !self.is_certain(Component::Year)
    }

    /// Get all certain components as a list
    pub fn get_certain_components(&self) -> Vec<Component> {
        let mut result = Vec::with_capacity(Component::COUNT);
        for i in 0..Component::COUNT {
            let comp = unsafe { std::mem::transmute::<u8, Component>(i as u8) };
            if self.is_certain(comp) {
                result.push(comp);
            }
        }
        result
    }

    /// Check if the date components are valid
    pub fn is_valid_date(&self) -> bool {
        use chrono::NaiveDate;

        let year = match self.get(Component::Year) {
            Some(y) => y,
            None => return false,
        };
        let month = match self.get(Component::Month) {
            Some(m) => m as u32,
            None => return false,
        };
        let day = match self.get(Component::Day) {
            Some(d) => d as u32,
            None => return false,
        };

        // Check if date is valid
        if NaiveDate::from_ymd_opt(year, month, day).is_none() {
            return false;
        }

        // Check time components if they exist
        if let Some(hour) = self.get(Component::Hour)
            && !(0..24).contains(&hour)
        {
            return false;
        }
        if let Some(minute) = self.get(Component::Minute)
            && !(0..60).contains(&minute)
        {
            return false;
        }
        if let Some(second) = self.get(Component::Second)
            && !(0..60).contains(&second)
        {
            return false;
        }

        true
    }

    /// Convert to a chrono DateTime
    pub fn to_datetime(
        &self,
        reference: &crate::ReferenceWithTimezone,
    ) -> Option<chrono::DateTime<chrono::Local>> {
        use chrono::{Local, NaiveDate, TimeZone};

        let year = self.get(Component::Year)?;
        let month = self.get(Component::Month)? as u32;
        let day = self.get(Component::Day)? as u32;
        let hour = self.get(Component::Hour).unwrap_or(12) as u32;
        let minute = self.get(Component::Minute).unwrap_or(0) as u32;
        let second = self.get(Component::Second).unwrap_or(0) as u32;

        let naive_dt =
            NaiveDate::from_ymd_opt(year, month, day)?.and_hms_opt(hour, minute, second)?;

        let local_dt = Local.from_local_datetime(&naive_dt).single()?;

        // Apply timezone offset adjustment if needed
        if let Some(offset) = self.get(Component::TimezoneOffset) {
            let adjustment = reference.get_system_timezone_adjustment(Some(local_dt), Some(offset));
            Some(local_dt + chrono::Duration::minutes(adjustment as i64))
        } else {
            Some(local_dt)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_flag() {
        assert_eq!(Component::Year.flag(), ComponentFlags::YEAR);
        assert_eq!(Component::Month.flag(), ComponentFlags::MONTH);
        assert_eq!(Component::Hour.flag(), ComponentFlags::HOUR);
    }

    #[test]
    fn test_fast_components_assign_and_get() {
        let mut comp = FastComponents::new();

        comp.assign(Component::Year, 2024);
        comp.assign(Component::Month, 11);
        comp.assign(Component::Day, 25);

        assert_eq!(comp.get(Component::Year), Some(2024));
        assert_eq!(comp.get(Component::Month), Some(11));
        assert_eq!(comp.get(Component::Day), Some(25));
        assert_eq!(comp.get(Component::Hour), None);

        assert!(comp.is_certain(Component::Year));
        assert!(!comp.is_certain(Component::Hour));
    }

    #[test]
    fn test_fast_components_imply() {
        let mut comp = FastComponents::new();

        comp.imply(Component::Hour, 12);
        assert_eq!(comp.get(Component::Hour), Some(12));
        assert!(!comp.is_certain(Component::Hour));

        // Assign should override imply
        comp.assign(Component::Hour, 15);
        assert_eq!(comp.get(Component::Hour), Some(15));
        assert!(comp.is_certain(Component::Hour));

        // Imply should not override assign
        comp.imply(Component::Hour, 9);
        assert_eq!(comp.get(Component::Hour), Some(15));
    }

    #[test]
    fn test_fast_components_copy() {
        let mut a = FastComponents::new();
        a.assign(Component::Year, 2024);

        // Copy should be cheap (no allocation)
        let b = a;

        assert_eq!(b.get(Component::Year), Some(2024));
    }

    #[test]
    fn test_size() {
        // Ensure the struct is reasonably sized (fits in cache line)
        assert!(std::mem::size_of::<FastComponents>() <= 64);
    }
}
