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

## Manual installation
1. Copy files into anywhere you want
2. Set JAVA_HOME Environment Variable to where you copy
   1. for Windows, See https://learn.microsoft.com/en-us/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14)#to-add-a-path-to-the-path-environment-variable
   2. for most Unix systems, See https://unix.stackexchange.com/a/26059 and keep in mind the sentence below,
   ````
   Put the line to modify PATH in ~/.profile, or in ~/.bash_profile or if that's what you have. (If your login shell is zsh and not bash, put it in ~/.zprofile instead.)
   ````
3. Append PATH Environment Variable with this 
   ````
   %JAVA_HOME% // bin MUST be excluded!
   ````
4. Reopen terminal to affect changes
5. Type it to check if it's installed successfully!

## Create installer

### for Windows
1. Install Inno setup 6
2. Open the program and jdk_selector_installer/windows/setup.iss
3. Compile it!
4. Check out Output folder in jdk_selector_installer/windows/Output

## TO-DOs

1. ~~Create draft source written in rust~~
2. ~~Create cross-platform builder in Rust~~
3. ~~Create Native installers for Windows~~
4. Create CI/CD for distribution for Windows
5. Check availability on macOS and Linux distributions (Tested on Windows 10 only at this moment)
6. Create Native installers and CI/CD for distribution for macOS/Linux
7. Create Unit test for better refactoring
8. Create document for welcomed PRs in future
9. Add missing executors for other Java versions (Verified on Java 8 and 17)