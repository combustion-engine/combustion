Combustion Game Engine
----------------------

A game engine written in Rust.

# Motivation

Because it's fun.

No, really. Rust is the most enjoyable language I've ever worked with, and even a massive undertaking
like a game engine is legitimately *fun* to work on.
 
Of course, I do intend to make a game on top of this eventually, and perhaps place this engine on my resume. 

# Work In Progress Warning

This project is a total mess right now. I've been rewriting and refining everything from the bottom up. This message will be removed whenever things get a bit more stable.

# Roadmap/Todo List

Currently the engine is in the stages of going from "I got some things working" to "I got things working in an flexible, organized and extensible way".

That is to say, I've been splitting up functionality into crates to simplify each component and to make compiling the engine more incremental.

## Completed or Working:

These are either fully working or in a state in which new features can be easily added. Their API is relatively stable. 

This does NOT mean all parts are feature-complete. Off the top of my head, animations need to be incorporated somehow.

- [x] Common utilities (`combustion_common` crate)
- [x] Engine protocols (`combustion_protocols` crate)
- [x] Asset importing/exporting (`combustion_asset` crate)

## TODO

- [ ] Design an API for the rendering backend
- [ ] Integrate scene graph, asset system and backend into engine core
- [ ] Create a modular shader generation pipeline
- [ ] Rewrite tooling with updated crates
- [ ] Figure out how to best handle animations

## Engine Crates

Most parts of the engine have been separated into multiple crates for both organizational purposes and to improve compile times.

All crates beginning with `combustion_` are integral parts of the engine. See each crate's README for specific information.

-----
## Developing on Windows (slightly outdated already):

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
        2. Select ONE of the Combustion subcrates, like `combustion_protocols`, `combustion_common`, `combustion_backend`, `combustion_core`, etc.
            * The Rust plugin only supports a single crate per project, so engine subcrates need to be opened as separate projects.
        3. Do that for all the subcrates you wish to work on. It will initialize an IDEA project in each so they can be reopened normally.
7. Building LuaJIT
    1. **TODO**: It's a pain.
    
## Developing on Linux

Although I am trying to write things in a platform agnostic manner, 
I haven't yet had a good chance to work on anything on Linux.

* **TODO**