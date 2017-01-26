@0x80f216b08d0ebb65;

using Math = import "/math.capnp";
using Util = import "/utils.capnp";

using Mesh = import "/mesh.capnp".Mesh;

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