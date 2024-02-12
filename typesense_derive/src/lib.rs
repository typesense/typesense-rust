use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Field, ItemStruct};

#[proc_macro_derive(Document, attributes(typesense))]
pub fn typesense_collection_derive(input: TokenStream) -> TokenStream {
    let item: ItemStruct = syn::parse(input).expect("Document can be only be derived for structs");

    // Build the trait implementation
    impl_typesense_collection(item).unwrap_or_else(|err| err.into_compile_error().into())
}

fn impl_typesense_collection(item: ItemStruct) -> syn::Result<TokenStream> {
    let name = &item.ident;
    let generics = add_trait_bounds(item.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let default_sorting_field = extract_default_sorting_field(&item.attrs)?;
    let collection_name =
        extract_collection_name(&item.attrs)?.unwrap_or_else(|| name.to_string().to_lowercase());

    let fields = if let syn::Fields::Named(syn::FieldsNamed { ref named, .. }, ..) = &item.fields {
        named
    } else {
        return Err(syn::Error::new_spanned(
            item.fields,
            "Document derive macro only can be used on structs with named fields.",
        ));
    };

    if let Some(ref sorting_field) = default_sorting_field {
        if !fields.iter().any(|field|
                // At this point we are sure that this field is a named field.
                field.ident.as_ref().unwrap() == sorting_field)
        {
            return Err(syn::Error::new_spanned(
                &item,
                "defined default_sorting_field does not match with any field.",
            ));
        }
    }

    let typesense_fields = fields
        .iter()
        .map(to_typesense_field_type)
        .collect::<syn::Result<Vec<_>>>()?;

    let gen = quote! {
        impl  #impl_generics  typesense::document::Document for #name #ty_generics #where_clause {
            fn collection_schema() -> typesense::collection::CollectionSchema {
                let name = #collection_name.to_string();

               let fields = vec![#(#typesense_fields,)*];

                let default_sorting_field = std::string::String::from(#default_sorting_field);

                typesense::collection::CollectionSchemaBuilder::new()
                   .name(name)
                   .fields(fields)
                   .default_sorting_field(default_sorting_field)
                   .build()
                   .unwrap()
            }
        }
    };
    Ok(gen.into())
}

// Get the inner type for a given wrappper
fn ty_inner_type<'a>(ty: &'a syn::Type, wrapper: &'static str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() == 1 && p.path.segments[0].ident == wrapper {
            if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
                if inner_ty.args.len() == 1 {
                    // len is 1 so this should not fail
                    let inner_ty = inner_ty.args.first().unwrap();
                    if let syn::GenericArgument::Type(ref t) = inner_ty {
                        return Some(t);
                    }
                }
            }
        }
    }
    None
}

// Add a bound `T: ToTypesenseField` to every type parameter T.
fn add_trait_bounds(mut generics: syn::Generics) -> syn::Generics {
    for param in &mut generics.params {
        if let syn::GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(syn::parse_quote!(typesense::field::ToTypesenseField));
        }
    }
    generics
}

// Iterate over all attributes and return the default field defined with the attribute:
//#[typesense_collection(default_sorting_field = "name_of_the_field")]
fn extract_default_sorting_field(attributes: &[Attribute]) -> syn::Result<Option<String>> {
    let default_sorting_field = attributes
        .iter()
        .filter_map(|attr| {
            if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "typesense" {
                if let Some(proc_macro2::TokenTree::Group(g)) =
                    attr.tokens.clone().into_iter().next()
                {
                    let mut tokens = g.stream().into_iter();
                    let attribute_tt = match tokens.next() {
                        Some(proc_macro2::TokenTree::Ident(ref i)) => {
                            if i == "default_sorting_field" {
                                i.clone()
                            } else if i == "collection_name" {
                                return None;
                            } else {
                                return Some(Err(syn::Error::new(
                                    i.span(),
                                    "expected: typesense(default_sorting_field  = \"...\")",
                                )));
                            }
                        }
                        Some(tt) => {
                            return Some(Err(syn::Error::new_spanned(
                                &tt,
                                format!("Unexpected {}", tt),
                            )));
                        }
                        None => {
                            return Some(Err(syn::Error::new_spanned(
                                attr,
                                "expected: typesense(default_sorting_field  = \"...\")",
                            )));
                        }
                    };
                    let eq_sign = match tokens.next() {
                        Some(proc_macro2::TokenTree::Punct(ref p)) => {
                            assert_eq!(p.as_char(), '=');
                            p.clone()
                        }
                        Some(tt) => {
                            return Some(Err(syn::Error::new_spanned(
                                &tt,
                                format!("Unexpected {}, expected equal sign(=)", tt),
                            )));
                        }
                        None => {
                            return Some(Err(syn::Error::new_spanned(
                                attribute_tt,
                                "expected: equal sign(=)",
                            )));
                        }
                    };
                    let default_sorting_field = {
                        let lit = match tokens.next() {
                            Some(proc_macro2::TokenTree::Literal(ref i)) => {
                                syn::Lit::new(i.clone())
                            }
                            Some(tt) => {
                                return Some(Err(syn::Error::new_spanned(
                                    &tt,
                                    format!("Expected string literal, do you mean \"{}\"?", tt),
                                )));
                            }
                            _ => {
                                return Some(Err(syn::Error::new_spanned(
                                    eq_sign,
                                    "Expected string literal",
                                )))
                            }
                        };
                        if let syn::Lit::Str(s) = lit {
                            s.value()
                        } else {
                            return Some(Err(syn::Error::new_spanned(
                                lit,
                                "default_sorting_field must be equal to a literal string",
                            )));
                        }
                    };
                    if let Some(tt) = tokens.next() {
                        return Some(Err(syn::Error::new_spanned(tt, "Unexpected token")));
                    }
                    Some(Ok(default_sorting_field))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<syn::Result<Vec<String>>>();
    match default_sorting_field {
        Ok(mut default_sorting_field) => {
            if default_sorting_field.len() <= 1 {
                Ok(default_sorting_field.pop())
            } else {
                Err(syn::Error::new_spanned(
                    // This will not fail since if we are at this point is because attributes is not empty.
                    attributes.first().unwrap(),
                    format!(
                        "Expected only one default_sorting_field, found {}",
                        default_sorting_field.len(),
                    ),
                ))
            }
        }
        Err(err) => Err(err),
    }
}

fn extract_collection_name(attributes: &[Attribute]) -> syn::Result<Option<String>> {
    let collection_name = attributes
        .iter()
        .filter_map(|attr| {
            if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "typesense" {
                if let Some(proc_macro2::TokenTree::Group(g)) =
                    attr.tokens.clone().into_iter().next()
                {
                    let mut tokens = g.stream().into_iter();
                    let attribute_tt = match tokens.next() {
                        Some(proc_macro2::TokenTree::Ident(ref i)) => {
                            if i == "collection_name" {
                                i.clone()
                            } else if i == "default_sorting_field" {
                                return None;
                            } else {
                                return Some(Err(syn::Error::new(
                                    i.span(),
                                    "expected: typesense(collection_name  = \"...\")",
                                )));
                            }
                        }
                        Some(tt) => {
                            return Some(Err(syn::Error::new_spanned(
                                &tt,
                                format!("Unexpected {}", tt),
                            )));
                        }
                        None => {
                            return Some(Err(syn::Error::new_spanned(
                                attr,
                                "expected: typesense(collection_name  = \"...\")",
                            )));
                        }
                    };
                    let eq_sign = match tokens.next() {
                        Some(proc_macro2::TokenTree::Punct(ref p)) => {
                            assert_eq!(p.as_char(), '=');
                            p.clone()
                        }
                        Some(tt) => {
                            return Some(Err(syn::Error::new_spanned(
                                &tt,
                                format!("Unexpected {}, expected equal sign(=)", tt),
                            )));
                        }
                        None => {
                            return Some(Err(syn::Error::new_spanned(
                                attribute_tt,
                                "expected: equal sign(=)",
                            )));
                        }
                    };
                    let collection_name = {
                        let lit = match tokens.next() {
                            Some(proc_macro2::TokenTree::Literal(ref i)) => {
                                syn::Lit::new(i.clone())
                            }
                            Some(tt) => {
                                return Some(Err(syn::Error::new_spanned(
                                    &tt,
                                    format!("Expected string literal, do you mean \"{}\"?", tt),
                                )));
                            }
                            _ => {
                                return Some(Err(syn::Error::new_spanned(
                                    eq_sign,
                                    "Expected string literal",
                                )))
                            }
                        };
                        if let syn::Lit::Str(s) = lit {
                            s.value()
                        } else {
                            return Some(Err(syn::Error::new_spanned(
                                lit,
                                "collection_name must be equal to a literal string",
                            )));
                        }
                    };
                    if let Some(tt) = tokens.next() {
                        return Some(Err(syn::Error::new_spanned(tt, "Unexpected token")));
                    }
                    Some(Ok(collection_name))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<syn::Result<Vec<String>>>();
    match collection_name {
        Ok(mut collection_name) => {
            if collection_name.len() <= 1 {
                Ok(collection_name.pop())
            } else {
                Err(syn::Error::new_spanned(
                    // This will not fail since if we are at this point is because attributes is not empty.
                    attributes.first().unwrap(),
                    format!(
                        "Expected only one collection_name, found {}",
                        collection_name.len(),
                    ),
                ))
            }
        }
        Err(err) => Err(err),
    }
}

/// Convert a given field in a typesense field type.
fn to_typesense_field_type(field: &Field) -> syn::Result<proc_macro2::TokenStream> {
    let name = &field.ident;

    let facet = {
        let facet_vec = field
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "typesense" {
                    if let Some(proc_macro2::TokenTree::Group(g)) =
                        attr.tokens.clone().into_iter().next()
                    {
                        let mut tokens = g.stream().into_iter();
                        match tokens.next() {
                            Some(proc_macro2::TokenTree::Ident(ref i)) => {
                                if i != "facet" {
                                    return Some(Err(syn::Error::new_spanned(
                                        i,
                                        format!("Unexpected token {}. Do you mean `facet`?", i),
                                    )));
                                }
                            }
                            Some(ref tt) => {
                                return Some(Err(syn::Error::new_spanned(
                                    tt,
                                    format!("Unexpected token {}. Do you mean `facet`?", tt),
                                )))
                            }
                            None => {
                                return Some(Err(syn::Error::new_spanned(attr, "expected `facet`")))
                            }
                        }

                        if let Some(ref tt) = tokens.next() {
                            return Some(Err(syn::Error::new_spanned(
                                tt,
                                "Unexpected token. Expected )",
                            )));
                        }
                        return Some(Ok(()));
                    }
                }
                None
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let facet_count = facet_vec.len();
        if facet_count == 1 {
            quote!(Some(true))
        } else if facet_count == 0 {
            quote!(None)
        } else {
            return Err(syn::Error::new_spanned(
                field,
                "#[typesense(facet)] repeated more than one time.",
            ));
        }
    };

    let (ty, optional) = if let Some(inner_ty) = ty_inner_type(&field.ty, "Option") {
        (inner_ty, quote!(Some(true)))
    } else {
        (&field.ty, quote!(None))
    };
    let typesense_field_type = quote!(
            <#ty as typesense::field::ToTypesenseField>::to_typesense_type().to_string()
    );
    Ok(quote! {
        typesense::field::FieldBuilder::new()
            .name(std::string::String::from(stringify!(#name)))
            .typesense_type(#typesense_field_type)
            .optional(#optional)
            .facet(#facet)
            .build()
            .unwrap()
    })
}
