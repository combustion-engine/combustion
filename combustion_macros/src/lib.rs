#![feature(proc_macro, proc_macro_lib)]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod ecs;

#[proc_macro_derive(Component, attributes(ecs))]
pub fn derive_ecs_component(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let ast = syn::parse_macro_input(&input).unwrap();

    let gen = ecs::component::impl_derive(&ast);

    gen.parse().expect("Failed to generate code")
}