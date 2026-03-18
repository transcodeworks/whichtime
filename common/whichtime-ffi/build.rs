fn main() {
    uniffi::generate_scaffolding("./src/whichtime.udl").expect("UniFFI scaffolding");
}
