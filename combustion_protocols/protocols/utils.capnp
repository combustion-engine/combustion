@0xa32f1c68bcd121ee;

# Equivalent to Rust's Option enum
struct Option(SomeType) {
    union {
        none @0: Void;
        some @1: SomeType;
    }
}

# Equivalent to C++ std::pair
struct Pair(FirstType, SecondType) {
    first @0: FirstType;
    second @1: SecondType;
}