@0x80f216b08d0ebb65;

using Math = import "/math.capnp";
using Tex = import "/texture.capnp";
using Util = import "/utils.capnp";

struct Model {
    root @0: Node;
    textures @1: List(Tex.Texture); # Embedded textures
}

struct Node {
    name @0: Text;
    meshes @1: List(Mesh);
    children @2: List(Node);
    transforms @3: List(Math.Transform);
}

struct Mesh {
    indices @0: Data;
    vertices @1: Data;
    uvs @2: Util.Option(Data);
    tangents @3: Util.Option(Data);
    bitangents @4: Util.Option(Data);
}

struct MaterialPropertyValue {
    property: union {
        roughness @0: Float32;
        metallic @1: Float32;
        # TODO: More supported properties
    }
}

struct MaterialProperty {
    name @0: Text;
    value: union {
        text @1: Text;
        blob @2: Data;
        value @3: MaterialPropertyValue;
    }
}

struct Material {
    properties @0: List(MaterialProperty);
}
