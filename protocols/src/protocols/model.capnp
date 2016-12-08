@0x80f216b08d0ebb65;

struct Model {
    meshes @0: List(Mesh);
}

struct Option(Type) {
    union {
        none @0: Void;
        some @1: Type;
    }
}

struct Mesh {
    indices @0: List(UInt64);
    vertices @1: List(UInt64);
    uvs @2: Option(List(UInt64));
}
