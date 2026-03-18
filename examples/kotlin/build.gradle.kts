plugins {
    kotlin("jvm") version "1.9.21"
    application
}

group = "works.transcode.whichtime"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    // JNA for native library loading (required by UniFFI)
    implementation("net.java.dev.jna:jna:5.14.0")
    
    // Include the generated Kotlin bindings
    // The bindings are generated to: kotlin/src/main/kotlin/
    // Copy them or add as source set
}

kotlin {
    jvmToolchain(17)
}

application {
    mainClass.set("works.transcode.whichtime.example.MainKt")
}

// Add the generated bindings as a source directory
sourceSets {
    main {
        kotlin {
            srcDir("../../kotlin/src/main/kotlin")
        }
    }
}

// Set the library path for the native library
tasks.named<JavaExec>("run") {
    // Point to the directory containing libwhichtime.dylib/.so
    systemProperty("jna.library.path", "${rootProject.projectDir}/../../target/release")
    
    // Alternative: set as environment variable
    // environment("DYLD_LIBRARY_PATH", "${rootProject.projectDir}/../../target/release")
}

