@0x80f216b08d0ebb65;

using Math = import "/math.capnp";
using Util = import "/utils.capnp";

struct Model {
    root @0: Node;
    meshes @1: List(Mesh);
}

struct Node {
    name @0: Text;
    meshes @1: List(UInt32);
    children @2: List(Node);
    transforms @3: List(Math.Transform);
}

struct Mesh {
    material @0: Util.Option(Text);
    indices @1: Data;
    vertices @2: Data;
    uvs @3: Util.Option(Data);
    normals @4: Util.Option(Data);
    tangents @5: Util.Option(Data);
    bitangents @6: Util.Option(Data);
}