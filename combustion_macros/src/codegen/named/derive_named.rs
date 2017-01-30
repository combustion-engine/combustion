use syn;
use quote;

pub fn expand(ast: &syn::MacroInput) -> Result<quote::Tokens, String> {
    let ident = &ast.ident;

    Ok(quote! {
        impl ::common::traits::Named for #ident {
            fn name(&self) -> &String { &self.name }

            #[inline]
            fn set_name(&mut self, name: String) {
                self.name = name;
            }
        }
    })
}