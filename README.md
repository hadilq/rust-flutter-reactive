
Rust Flutter Reactive
---
This is a sample app to improve consistency over Mobile App Development. You can find more explanation below. By the way, I started playing with [fluterust](https://github.com/shekohex/flutterust) sample, but in the end, by reorganize its structure and architecture, and also don't rely on FFI code generations, come up with this solution, so part of the code here is just a copy of that sample.

Technologies
---
The followings are the libraries and technologies that are used in this sample.
 - Rust as the main part of the app.
 - Flutter as the ui framework. You can find the flutter project in `ui` directory.
 - Clean Architecture as you can find the domain, data, and presentation layers in `domain`, `data`, and `presentaiton` directories. The data layer is completely empty and just created to show how wire up these layers.
 - FFI as Foreign function interface and bridge between Rust and Dart code in Flutter. You can find this bridge in `bridge-ffi` directory.
 - Flatbuffers as the serializer of the bridge. Thanks to fast and scalable serialization of Flatbuffers, this project can be easily plugged out of Flutter and attached to other ui frameworks if exists! Also it make this project scalable without touching the FFI bridge code(Just in case of error or memory leak you may need to touch it). Flatbuffers generates the `state` and `action`, where both of them are tree structures, so each developer can work on their branches without conflicting with other developers. Scalability is in the heart of this sample.
 - Redux as the architectural pattern of presentation layer. It designed to be fractal(The tree structure that mentioned before), so it's highly scalable among large number of developers.

Enjoy!

Why Rust?
---
I can mention the following for my choice.
 - Performance for sure.
 - Strong typing up to constant generics, which means scalability. This one is highly important in my mind, as it can makes you 10x developer by avoiding future bugs and inconsistencies. By up to constant generics, I mean its type system is stronger than Java, Kotlin, Swift, etc. so the code base will be more scalable in Rust.
 - Super fast compiling when you re-run the tests.
 - Compiler messages are more like Stackoverflow posts. Most of them guide you to the correct solution.
 - The same as Dart it makes the code independent of platform.
 
Why Flutter?
---
It makes the code independent of the platform. As an Android developer, we face solving problems of Android fragmentation, much frequently than it should be. Add to these inconsistencies, every company deals at least with two sets of developers, Android and iOS developers, for the same problem. If you have the same use-case/validation in backend and frontend you have more than two sets of developers for the same problem. This inconsistencies in worst cases produce bugs. But even when there is no bug, they make users angry why the same feature is working differently in two platforms. By making your code independent of platform you can make it consistent. However, everything is a trade-off. The price you pay for consistency among all platforms is the size of the app. For instance, the size of the `app-debug.apk` file of this sample is 38M, which I think worth it.

Run/Test
---
To build and run tests just use standard `cargo` commands.
```bash
cargo test
```
Before making the artifacts from rust code first run
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
rustup target add aarch64-apple-ios x86_64-apple-ios
cargo install cargo-make
```
Then build the artifacts by
```bash
cargo make
```
To run the flutter app
```bash
cd ui && flutter run
```

Questions/Problems
---
Please feel free to fill an issue to ask questions or report issues. Also creating PRs are welcomed.
