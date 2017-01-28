@0xf53f070a70eb22d7;

# Simple 3D Vector structure
struct Vector3 {
    x @0: Float32;
    y @1: Float32;
    z @2: Float32;
}

# Simple 3D Point structure
struct Point3 {
    x @0: Float32;
    y @1: Float32;
    z @2: Float32;
}

# 4x4 square matrix structure
struct Matrix4 {
    m11 @0: Float32;
    m21 @1: Float32;
    m31 @2: Float32;
    m41 @3: Float32;
    m12 @4: Float32;
    m22 @5: Float32;
    m32 @6: Float32;
    m42 @7: Float32;
    m13 @8: Float32;
    m23 @9: Float32;
    m33 @10: Float32;
    m43 @11: Float32;
    m14 @12: Float32;
    m24 @13: Float32;
    m34 @14: Float32;
    m44 @15: Float32;
}

# Union of potential 3D transforms
struct Transform {
    transform: union {
        translation @0: Vector3;
        rotation @1: Vector3;
        scale @2: Vector3;
        matrix @3: Matrix4;
    }
}
