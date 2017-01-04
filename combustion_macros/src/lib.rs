#![feature(proc_macro, proc_macro_lib)]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate matches;

use proc_macro::TokenStream;

mod codegen;

#[proc_macro_derive(Component, attributes(ecs))]
pub fn derive_ecs_component(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let ast = syn::parse_macro_input(&input).unwrap();

    codegen::ecs::component_derive::expand_derive(&ast)
        .expect("Failed to run codegen")
        .parse()
        .expect("Failed to generate code")
}