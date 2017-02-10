`combustion_protocols`
----------------------

Combustion defines a "Protocol" as an API for interacting with data on external storage mediums, like the filesystem or networks.

Typically a protocol is composed of serializable and deserializable data structures and/or an API to facilitate serialization/deserialization functionality.

The two most common protocol kinds used in `combustion_protocols` are 
[Cap'N Proto](https://capnproto.org/index.html) schema protocols, and automatic protocols via Serde.

Generally, Cap'N Proto is used for binary formats, while Serde is mostly for textual human-readable formats like YAML or JSON.

## Features:

- [x] Scenes
    - [x] Lights
- [x] Models
- [x] Textures
    - [x] Uncompressed and Compressed
- [x] Materials
- [x] All (De)Serializable via Serde