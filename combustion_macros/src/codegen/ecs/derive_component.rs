use syn;
use quote;

struct ECSComponentProperties {
    storage_path: Option<syn::Path>,
    builtin_storage: bool,
    ecs_path: Option<syn::Path>,
}

pub fn expand(ast: &syn::MacroInput) -> Result<quote::Tokens, String> {
    let mut props = ECSComponentProperties {
        storage_path: None,
        builtin_storage: true,
        ecs_path: None,
    };

    let name = &ast.ident;
    let attrs = &ast.attrs;

    for attr in attrs {
        match attr.value {
            // #[ecs]
            syn::MetaItem::List(ref ident, ref nested) if ident == "ecs" => {
                for attr in nested {
                    match attr {
                        // #[ecs(...)]
                        &syn::NestedMetaItem::MetaItem(ref meta) => {
                            match meta {
                                // #[ecs(ident = lit)]
                                &syn::MetaItem::NameValue(ref ident, ref lit) => {
                                    match ident.as_ref() {
                                        // #[ecs(storage = "VecStorage")]
                                        "storage" => {
                                            if let &syn::Lit::Str(ref s, _) = lit {
                                                props.storage_path = Some(syn::parse_path(s.as_str())?);

                                                props.builtin_storage = matches!(s.as_str(), "VecStorage" | "HashMapStorage" | "NullStorage");
                                            }
                                        }
                                        // #[ecs(path = "combustion_ecs")]
                                        "path" => {
                                            if let &syn::Lit::Str(ref s, _) = lit {
                                                props.ecs_path = Some(syn::parse_path(s.as_str())?);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let ecs_path = props.ecs_path.unwrap_or_else(|| { syn::parse_path("::ecs").unwrap() });

    let mut storage_path = props.storage_path.unwrap_or_else(|| { syn::parse_path("VecStorage").unwrap() });

    let mut component_path = syn::Path::from("Component");

    for segment in ecs_path.segments.iter() {
        component_path.segments.insert(0, segment.clone());
    }

    component_path.global = ecs_path.global;

    if props.builtin_storage {
        for segment in ecs_path.segments.iter() {
            storage_path.segments.insert(0, segment.clone());
        }

        storage_path.global = ecs_path.global;
    }

    Ok(quote! {
        impl #component_path for #name {
            type Storage = #storage_path<#name>;
        }
    })
}