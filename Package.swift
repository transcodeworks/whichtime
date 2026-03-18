// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

// Whether to use a locally built XCFramework during development or a remote artifact from GitHub Releases.
// For now, we default to the local framework so you can iterate inside this repo.
let useLocalFramework = true

let binaryTarget: Target

if useLocalFramework {
  binaryTarget = .binaryTarget(
    name: "WhichtimeCoreRS",
    // Built by common/build-ios.sh from the whichtime-ffi crate.
    path: "./target/ios/libwhichtime_ffi-rs.xcframework"
  )
} else {
  // When you have automated releases set up, publish a zipped XCFramework to GitHub Releases
  // and update the tag and checksum here to match.
  let releaseTag = "0.1.0"
  let releaseChecksum = "<#xcframework-zip-checksum#>"
  binaryTarget = .binaryTarget(
    name: "WhichtimeCoreRS",
    url:
      "https://github.com/transcodeworks/whichtime/releases/download/\(releaseTag)/libwhichtime_ffi-rs.xcframework.zip",
    checksum: releaseChecksum
  )
}

let package = Package(
  name: "Whichtime",
  defaultLocalization: "en",
  platforms: [
    .iOS(.v16),
    .macOS(.v13),
  ],
  products: [
    // Library product that clients can depend on from SwiftPM.
    .library(
      name: "Whichtime",
      targets: ["Whichtime"]
    )
  ],
  dependencies: [
    // Add Swift-only testing or helper dependencies here (e.g. snapshot testing) as needed.
  ],
  targets: [
    // Binary target wrapping the Rust XCFramework built from the whichtime crate.
    binaryTarget,

    // UniFFI-generated Swift bindings.
    .target(
      name: "Whichtime",
      dependencies: [.target(name: "WhichtimeCoreRS")],
      path: "swift/Sources/UniFFI"
    ),

    // MARK: - Testing (to be filled in later)
    //
    // .testTarget(
    //     name: "WhichtimeCoreTests",
    //     dependencies: ["WhichtimeCoreFFI"],
    //     path: "apple/Tests/WhichtimeCoreTests"
    // ),
  ]
)
