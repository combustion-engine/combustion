@0xf063134c28cffff2;

using Math = import "/math.capnp";
using Util = import "/utils.capnp";

# Simple UV texture coordinates
struct TexCoord {
    u @0: Float32;
    v @1: Float32;
}

# Describes a single interleaved vertex
struct Vertex {
    position    @0: Math.Point3;
    normal      @1: Math.Vector3;
    uv          @2: TexCoord;
}

# Describes discrete vertex data, where data is NOT interleaved
#
# The components of this MUST be analogous to the above Vertex structure,
# just in discrete lists
struct Vertices {
    positions   @0: List(Math.Point3);
    normals     @1: Util.Option(List(Math.Vector3));
    uvs         @2: Util.Option(List(TexCoord));
}

# Like Vertices, but isn't type-safe
struct VerticesRaw {
    positions   @0: Data;
    normals     @1: Util.Option(Data);
    uvs         @2: Util.Option(Data);
}

# The Mesh structure, which defines materials, vertex data and optionally vertex indices.
struct Mesh {
    # List of materials for the given mesh. Materials are layered in the order given.
    # The values of this list are actually the indexes for the materials in the `Model` structure.
    materials @0: List(UInt32);

    vertices: union {
        interleaved     @1: List(Vertex); # Interleaved type-safe vertex data
        discrete        @2: Vertices;     # Discrete type-safe vertex data
        interleavedRaw  @4: Data;         # Interleaved UNSAFE vertex data
        discreteRaw     @5: VerticesRaw;  # Discrete UNSAFE vertex data
    }

    indices @3: Util.Option(List(UInt32));
}
