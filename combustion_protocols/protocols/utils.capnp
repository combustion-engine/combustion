@0xa32f1c68bcd121ee;

# Equivalent to Rust's Option enum
struct Option(Type) {
    union {
        none @0: Void;
        some @1: Type;
    }
}