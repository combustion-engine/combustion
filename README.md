Combustion Game Engine
----------------------

## Developing on Windows:

##### Dependencies:
* [MSVC 2015 with C++ components](https://www.visualstudio.com/vs/cplusplus/) (Community Edition should work)
* [Rapid Environment Editor](https://www.rapidee.com/en/about)
* [Git](https://git-scm.com/downloads)
* [IntelliJ IDEA](https://www.jetbrains.com/idea/)
    * [Jetbrains CLion](https://www.jetbrains.com/clion/) only if you want to do C++ development
* [MSYS2](https://msys2.github.io/) for *nix command line tools
* [CMake](https://cmake.org/download/)
* [rustup](https://www.rustup.rs/) to actually install Rust

##### Recommend tools (not required):
* [Sublime Text](https://www.sublimetext.com/)
* [Cmder](http://cmder.net/) (Mini version)

#### Getting Started

1. Install all the above dependencies, and use Rapid Environment Editor to make sure all the requires programs are in your `PATH`.
2. Clone Combustion repository somewhere using `git clone --recursive git@gitlab.com:combustion/combustion.git`
    * Good idea to enable NTFS compression wherever you clone it.
3. Open a console window (assuming you've added `rustup` to your `PATH`) and run these commands:
    1. `rustup toolchain install nightly-x86_64-pc-windows-msvc`
    2. `rustup component add rust-src`
    3. `rustup default nightly-x86_64-pc-windows-msvc`
4. Add these paths to your `PATH` (probably using Rapid Environment Editor)
    * `C:\Users\<USER>\.cargo\bin`
    * `C:\Program Files\CMake\bin`
    * Your `MSYS2` `usr/bin` directory
    * If not already added, the Git `bin` directory
    * The `bin` directory from the Combustion engine repo.
    * **NOTE:** Windows does not update the `PATH` variable for running processes. Cmd prompts and programs must be restarted for `PATH` changes to take effect. 
5. Crate a new environmental variable named `RUST_SRC_PATH`
    * Set `RUST_SRC_PATH=C:\Users\<USER>\.multirust\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\src`
        * Or similar
6. Start up IntelliJ IDEA
    1. Go to Settings -> Plugins -> Browse Repositories -> Manage repositories
    2. Add these repositories:
        - https://plugins.jetbrains.com/plugins/nightly/8182
        - https://plugins.jetbrains.com/plugins/nightly/8195
    3. Go back to Browse Repositories
    4. Install `Rust` and `TOML` nightly plugins
    5. Import project from sources
        1. Select Import project from existing sources
        2. Select ONE of the Combustion subcrates, like `protocols`, `common`, `backend` or `engine`.
            * The Rust plugin only supports a single crate per project, so engine subcrates need to be opened as separate projects.
        3. Do that for all the subcrates you wish to work on. It will initialize an IDEA project in each so they can be reopened normally.
7. Building LuaJIT
    1. **TODO**: It's a pain.
    
## Developing on Linux

* **TODO**