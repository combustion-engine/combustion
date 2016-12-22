#!/usr/bin/env bash
find . | grep -E "\.(glsl|rs|capnp|toml)$" | grep -vE "(target|/gl_bindings|external|deps/(specs|nalgebra|lz4-rs|capnpc-rust))" | xargs wc -l
