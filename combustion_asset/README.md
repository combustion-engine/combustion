`combustion_asset`
------------------

Asset importing, exporting and conversion routines.

## Features

- [x] Models
    * Import and export:
        - [x] Combustion model format
    * Import only:
        - [x] External models via Assimp
- [x] Textures
    * Import and export:
        - [x] Combustion texture format
        - [x] External images via the `image` crate
- [x] Virtual File System support
    - [x] Standard files
    - [x] `/dev/null`-like VFS
    - [x] Read-only memory mapped files