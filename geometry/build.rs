extern crate gcc;

use std::env;

const INCLUDES: &'static [&'static str] = &[
    "lib/libigl/include",
    "lib/eigen"
];

const SOURCE_FILES: &'static [&'static str] = &[
    "src/base.cpp"
];

const GCC_FLAGS: &'static [&'static str] = &[
    "-std=c++11",
    "-pedantic",
    "-fno-check-new",
    "-fno-common",
    "-fstrict-aliasing",
    "-Wall",
];

const MSVC_FLAGS: &'static [&'static str] = &[
    "/wd4127", "/wd4505", "/wd4714",
];

const MSVC_RELEASE_FLAGS: &'static [&'static str] = &[
    "/Ox", "/Oi", "/Qpar", "/openmp",
];

fn main() {
    let mut config = gcc::Config::new();

    config.cpp(true);

    for include in INCLUDES { config.include(include); }

    for source_file in SOURCE_FILES { config.file(source_file); }

    let target = env::var("TARGET").unwrap();
    let opt_level = env::var("OPT_LEVEL").unwrap();

    if target.contains("gcc") {
        for flag in GCC_FLAGS { config.flag(flag); }
    } else if target.contains("msvc") {
        for flag in MSVC_FLAGS { config.flag(flag); }

        match opt_level.as_str() {
            "3" | "2" => {
                for flag in MSVC_RELEASE_FLAGS { config.flag(flag); }
            },
            "1" => {},
            "z" | "s" => {},
            _ => {}
        }
    }

    config.compile("libgeometry.a");
}
