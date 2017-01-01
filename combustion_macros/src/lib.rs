#![feature(proc_macro, proc_macro_lib)]
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

// Cannot be public
mod hello;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = hello::impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}