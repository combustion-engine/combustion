@0x80f216b08d0ebb65;

using Math = import "/math.capnp";
using Util = import "/utils.capnp";

using Mesh = import "/mesh.capnp".Mesh;

struct RootModel {
    model @0: Model;
}

struct Model {
    root        @0: Node;       # Root node
    meshes      @1: List(Mesh); # List of meshes in the model
    materials   @2: List(Text); # List of materials used in this model
}

struct Node {
    name        @0: Text;
    meshes      @1: List(UInt32);
    children    @2: List(Node);

    # Transforms to apply to node members, in order
    transforms  @3: List(Math.Transform);
}