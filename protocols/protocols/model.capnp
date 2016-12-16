@0x80f216b08d0ebb65;

using Math = import "/math.capnp";

# Equivalent to Rust's Option enum
struct Option(Type) {
    union {
        none @0: Void;
        some @1: Type;
    }
}

struct Model {
    root @0: Node;
}

struct Transform {
    transform: union {
        translate @0: Math.Vector3;
        rotation @1: Math.Vector3;
        matrix @2: Math.Matrix4;
    }
}

struct Node {
    name @0: Text;
    meshes @1: List(Mesh);
    children @2: List(Node);
    transforms @3: List(Transform);
}

struct Mesh {
    indices @0: Data;
    vertices @1: Data;
    uvs @2: Option(Data);
    tangents @3: Option(Data);
    bitangents @4: Option(Data);

    material @5: Option(Material);
}

struct Material {

}
