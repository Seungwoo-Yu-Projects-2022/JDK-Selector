# JDK-Selector
`Select your JDK anywhere, anytime by single or two commands`

### This library provides you opportunity to change JDK version instantly by some commands.

## Build steps
1. Install rustup https://rustup.rs/
2. Clone this source
    ````
   git clone https://github.com/Seungwoo-Yu/JDK-Selector
    ````
3. Run cargo update in order to download dependencies
   ````
   cargo update
   ````
4. Build source
   ````
   cargo run --bin jdk_selector_builder
   ````
5. Check out .build folder in the source root!

## TO-DOs

1. ~~Create draft source written in rust~~
2. ~~Create cross-platform builder in Rust~~
3. Create Native installers and CI/CD for distribution for Windows
4. Check availability on macOS and Linux distributions (Tested on Windows 10 only at this moment)
5. Create Native installers and CI/CD for distribution for macOS/Linux
6. Create Unit test for better refactoring
7. Create document for welcomed PRs in future
8. Add missing executors for other Java versions (Verified on Java 8 and 17)