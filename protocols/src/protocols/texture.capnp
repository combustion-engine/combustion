@0xa0647fd03678a3a6;

enum Compression {
    none @0;
}

enum Format {
    rgb @0;
    rgba @1;
    luma @2;
    lumaAlpha @3;
}

struct Texture {
    width @0: UInt32;
    height @1: UInt32;
    format @2: Format;
    compression @3: Compression;
    data @4: Data;
}
