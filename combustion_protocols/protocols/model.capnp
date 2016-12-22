@0x80f216b08d0ebb65;

using Math = import "/math.capnp";
using Util = import "/utils.capnp";

struct Model {
    # Root node
    root @0: Node;

    # List of meshes in the model
    meshes @1: List(Mesh);

    # List of materials used in this model
    materials @2: List(Text);
}

struct Node {
    name @0: Text;
    meshes @1: List(UInt32);
    children @2: List(Node);

    # Transforms to apply to node members, in order
    transforms @3: List(Math.Transform);
}

struct Mesh {
    # List of materials for the given mesh. Materials are layered in the order given.
    # The values of this list are actually the indexes for the materials in the `Model` structure.
    materials @0: List(UInt32);

    indices @1: Data;
    vertices @2: Data;
    uvs @3: Util.Option(Data);
    normals @4: Util.Option(Data);
}