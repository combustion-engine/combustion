@0xa0647fd03678a3a6;

# https://www.opengl.org/wiki/Red_Green_Texture_Compression
enum Rgtc {
    red         @0; # Unsigned normalized 1-component
    redSigned   @1; # Signed normalized   1-component
    rg          @2; # Unsigned normalized 2-component
    rgSigned    @3; # Signed normalized   2-component
}

# https://www.opengl.org/wiki/BPTC_Texture_Compression
enum Bptc {
    rgba                @0; # Unsigned normalized 4-components
    rgbFloatSigned      @1; # Signed, floating-point 3-components.
    rgbFloatUnsigned    @2; # Unsigned, floating-point 3-components.
}

# https://www.opengl.org/wiki/S3_Texture_Compression
enum S3tc {
    rgb1   @0;  # RGB DXT1
    rgba1  @1;  # RGBA DXT1
    rgba3  @2;  # RGBA DXT3
    rgba5  @3;  # RGBA DXT5
}

# ASTC Block size
# https://www.opengl.org/wiki/ASTC_Texture_Compression
enum BlockSize {
    b4x4    @0;
    b5x4    @1;
    b5x5    @2;
    b6x5    @3;
    b6x6    @4;
    b8x5    @5;
    b8x6    @6;
    b10x5   @7;
    b10x6   @8;
    b8x8    @9;
    b10x8   @10;
    b10x10  @11;
    b12x10  @12;
    b12x12  @13;
}

# Raw Uncompressed Pixel format
enum Channels {
    r           @0;
    rg          @1;
    rgb         @2;
    rgba        @3;
}

enum DataType {
    unsignedByte @0;            # UNSIGNED_BYTE
    byte @1;                    # BYTE
    unsignedShort @2;           # UNSIGNED_SHORT
    short @3;                   # SHORT
    unsignedInt @4;             # UNSIGNED_INT
    int @5;                     # INT
    float @6;                   # FLOAT
    unsignedByte332 @7;         # UNSIGNED_BYTE_3_3_2
    unsignedByte233Rev @8;      # UNSIGNED_BYTE_2_3_3_REV
    unsignedShort565 @9;        # UNSIGNED_SHORT_5_6_5
    unsignedShort565Rev @10;    # UNSIGNED_SHORT_5_6_5_REV
    unsignedShort4444 @11;      # UNSIGNED_SHORT_4_4_4_4
    unsignedShort4444Rev @12;   # UNSIGNED_SHORT_4_4_4_4_REV
    unsignedShort5551 @13;      # UNSIGNED_SHORT_5_5_5_1
    unsignedShort1555Rev @14;   # UNSIGNED_SHORT_1_5_5_5_REV
    unsignedInt8888 @15;        # UNSIGNED_INT_8_8_8_8
    unsignedInt8888Rev @16;     # UNSIGNED_INT_8_8_8_8_REV
    unsignedInt1010102 @17;     # UNSIGNED_INT_10_10_10_2
    unsignedInt2101010Rev @18;  # UNSIGNED_INT_2_10_10_10_REV

    # No data type given. This will assume `unsignedByte` most likely, depending on the situation
    unspecified @19;
}

struct Uncompressed {
    format @0: Channels;
    type @1: DataType;
}

enum TextureKind {
    texture1D @0;
    texture2D @1;
    texture3D @2;
}

struct Texture {
    # Dimensions of the texture
    width   @0: UInt32 = 0;
    height  @1: UInt32 = 0;

    # Compression method
    compression: union {
        none @2: Uncompressed;  # Uncompressed
        rgtc @3: Rgtc;          # Compressed using RGTC
        bptc @4: Bptc;          # Compressed using BPTC
        astc @5: BlockSize;     # Compressed using ASTC
        s3tc @6: S3tc;          # Compressed using S3TC/DXT
    }

    data  @7: Data;                     # Binary texture data
    srgb  @8: Bool;                     # sRGB support
    depth @9: UInt32 = 0;               # Depth for 3D textures
    kind  @10: TextureKind = texture2D; # 1D, 2D, 3D, etc

    # All Mipmaps are assumed to be the same format as the original texture. Level is given by the list index plus one
    mipmaps @11: List(Data);            # Mipmaps.
}

struct Cubemap {
    right   @0: Texture; # Positive X
    left    @1: Texture; # Negative X
    top     @2: Texture; # Positive Y
    bottom  @3: Texture; # Negative Y
    back    @4: Texture; # Positive Z
    front   @5: Texture; # Negative Z
}

struct RootTexture {
    texture: union {
        single @0: Texture;
        cubemap @1: Cubemap;
    }
}