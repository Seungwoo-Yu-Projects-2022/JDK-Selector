# JDK-Selector
`Select your JDK anywhere, anytime by single or two commands`

# This project is discontinued because...
It was my first project in Rust and kind of draft so codes didn't look clean, straightforward and cover edge cases at most times.

Moreover, I felt this project could be used in more places than where I expected.

Therefore, to expand concept of project I decided to create and maintain another well-designed project.

Check repo: https://github.com/Seungwoo-Yu/Package-Select

JDK-Selector still does the job well on Windows and Linux (Amd64 only though) and arm64 macOS.

---

### This library provides you opportunity to change JDK version instantly by some commands.

## Simple usage
1. Install JDK-Selector
2. (Re)open terminal
3. Type `jdk_selector_cli add name path`
   ````
   i.e jdk_selector_cli add "JDK 1.8" "C:\Program Files\Java\jdk1.8.0_321"
   ````
4. Type `jdk_selector_cli use name`
   ````
   i.e jdk_selector_cli use "JDK 1.8"
   ````
5. Done!

For details, please see `jdk_selector_cli (help)`

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
5. Check out `.build` folder in the source root!

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
5. Type this to check if it's installed successfully!
   ````
   jdk_selector_cli help
   ````

## Create installer

### for Windows
1. Install Inno setup 6
2. Open the program and `jdk_selector_installer/windows/setup.iss`
3. Compile it!
4. Check out Output folder in `jdk_selector_installer/windows/Output`

## TO-DOs

1. ~~Create draft source written in rust~~
2. ~~Create cross-platform builder in Rust~~
3. ~~Create Native installers for Windows~~
4. Create Native installers for Debian and Redhat distributions (Working...)
5. Add multi-architecture support for jdk_selector_builder and jdk_selector_installer/linux
6. Create CI/CD for distribution for Windows/Linux
7. Check availability on macOS distributions (Tested on Windows 10 and Ubuntu 22.04 only at this moment)
8. Create Native installers and CI/CD for distribution for macOS
9. Create Unit test for better refactoring
10. Create document for welcomed PRs in future
11. Add missing executors for other Java versions (Verified on Oracle Java 8 and 17 on Windows 10 and OpenJDK Java 8 on Ubuntu 22.04)
