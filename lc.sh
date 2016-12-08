find . | grep -E "\.(glsl|rs|capnp|toml)$" | grep -vE "(target|/gl_bindings|external)" | xargs wc -l
