extern "C" {
    fn test() -> i32;
}

fn main() {
    println!("Hello, world! {}", unsafe { test() });
}
