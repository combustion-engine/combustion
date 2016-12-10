@0xa0647fd03678a3a6;

# https://www.opengl.org/wiki/Red_Green_Texture_Compression
enum Rgtc {
    red @0;         # Unsigned normalized 1-component
    redSigned @1;   # Signed normalized   1-component
    rg @2;          # Unsigned normalized 2-component
    rgSigned @3;    # Signed normalized   2-component
}

# https://www.opengl.org/wiki/BPTC_Texture_Compression
enum Bptc {
    rgba @0;        # Unsigned normalized 4-components
    srgba @1;       # Unsigned normalized 4-components in the sRGB colorspace.
    rgbFloat @2;    # Signed, floating-point 3-components.
    rgbaFloat @3;   # Unsigned, floating-point 3-components.
}

# https://www.opengl.org/wiki/S3_Texture_Compression
enum S3tc {
    rgb1   @0;  # RGB DXT1
    rgba1  @1;  # RGBA DXT1
    rgba3  @2;  # RGBA DXT3
    rgba5  @3;  # RGBA DXT5
    srgb1  @4;  # sRGB DXT1
    srgba1 @5;  # sRGBA DXT1
    srgba3 @6;  # sRGBA DXT3
    srgba5 @7;  # sRGBA DXT5
}

# ASTC Block size
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

# https://www.opengl.org/wiki/ASTC_Texture_Compression
struct Astc {
    blockSize @0: BlockSize;
    srgb @1: Bool; # Simple boolean for sRGB encoding
}

# Uncompressed Pixel format
enum Format {
    rgb @0;
    rgba @1;
    luma @2;
    lumaAlpha @3;
}

struct Texture {
    # Dimensions of the texture
    width @0: UInt32;
    height @1: UInt32;
    # Compression method
    compression: union {
        none @2: Format;    # Uncompressed data in the given format
        rgtc @3: Rgtc;      # Compressed using RGTC
        bptc @4: Bptc;      # Compressed using BPTC
        astc @5: Astc;      # Compressed using ASTC
        s3tc @6: S3tc;      # Compressed using S3TC/DXT
    }
    # Binary texture data
    data @7: Data;
}
