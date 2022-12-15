# bdk-android
This project builds an .aar package for the Android platform that provide Kotlin language bindings for the [`bdk`] library. The Kotlin language bindings are created by the [`bdk-ffi`] project which is included in the root of this repository.

## How to Use
To use the Kotlin language bindings for [`bdk`] in your Android project add the following to your gradle dependencies:
```kotlin
repositories {
    mavenCentral()
}

dependencies {
  implementation("org.bitcoindevkit:bdk-android:<version>")
}
```

You may then import and use the `org.bitcoindevkit` library in your Kotlin code. For example:
```kotlin
import org.bitcoindevkit.*

// ...

val externalDescriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
val internalDescriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"

val databaseConfig = DatabaseConfig.Memory

val blockchainConfig = BlockchainConfig.Electrum(
        ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5u, null, 10u)
    )
val wallet = Wallet(externalDescriptor, internalDescriptor, Network.TESTNET, databaseConfig, blockchainConfig)
val newAddress = wallet.getAddress(AddressIndex.LAST_UNUSED)
```

### Snapshot releases
To use a snapshot release, specify the snapshot repository url in the `repositories` block and use the snapshot version in the `dependencies` block:
```kotlin
repositories {
    maven("https://s01.oss.sonatype.org/content/repositories/snapshots/")
}

dependencies {
  implementation("org.bitcoindevkit:bdk-android:<version-SNAPSHOT>")
}
```

### Example Projects
* [Devkit Wallet](https://github.com/thunderbiscuit/devkit-wallet)
* [Padawan Wallet](https://github.com/thunderbiscuit/padawan-wallet)

### How to build
_Note that Kotlin version `1.6.10` or later is required to build the library._

1. Clone this repository.
```shell
git clone https://github.com/bitcoindevkit/bdk-ffi
```
2. Follow the "General" bdk-ffi ["Getting Started (Developer)"] instructions.
3. If building on macOS install required intel and m1 jvm targets
4. Install required targets
 ```sh
 rustup target add x86_64-linux-android aarch64-linux-android armv7-linux-androideabi
 ```
5. Install Android SDK and Build-Tools for API level 30+
6. Setup `$ANDROID_SDK_ROOT` and `$ANDROID_NDK_ROOT` path variables (which are required by the
   build tool), for example (NDK major version 21 is required):
 ```shell
 export ANDROID_SDK_ROOT=~/Android/Sdk
 export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/21.<NDK_VERSION>
 ```
7. Build kotlin bindings
 ```sh
 # build Android library
 cd bdk-android
 ./gradlew buildAndroidLib
 ```
8. Start android emulator (must be x86_64) and run tests
```sh
./gradlew connectedAndroidTest
```

## How to publish to your local Maven repo
```shell
cd bdk-android
./gradlew publishToMavenLocal --exclude-task signMavenPublication
```

Note that the commands assume you don't need the local libraries to be signed. If you do wish to sign them, simply set your `~/.gradle/gradle.properties` signing key values like so:
```properties
signing.gnupg.keyName=<YOUR_GNUPG_ID>
signing.gnupg.passphrase=<YOUR_GNUPG_PASSPHRASE>
```

and use the `publishToMavenLocal` task without excluding the signing task:
```shell
./gradlew publishToMavenLocal
```

[`bdk`]: https://github.com/bitcoindevkit/bdk
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi

temporary change to run the tests