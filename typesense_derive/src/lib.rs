use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{ToTokens, quote};
use syn::{Attribute, Field, ItemStruct, spanned::Spanned};

#[proc_macro_derive(Typesense, attributes(typesense))]
pub fn typesense_collection_derive(input: TokenStream) -> TokenStream {
    let item: ItemStruct = syn::parse(input).expect("Typesense can be only be derived for structs");

    // Build the trait implementation
    impl_typesense_collection(item).unwrap_or_else(|err| err.into_compile_error().into())
}

fn impl_typesense_collection(item: ItemStruct) -> syn::Result<TokenStream> {
    let item_ts = item.to_token_stream();

    let ItemStruct {
        attrs,
        vis: _,
        struct_token: _,
        ident,
        generics,
        fields,
        semi_token: _,
    } = item;

    let fields = if let syn::Fields::Named(syn::FieldsNamed { named, .. }, ..) = fields {
        named
    } else {
        return Err(syn::Error::new_spanned(
            fields,
            "Typesense derive macro only can be used on structs with named fields.",
        ));
    };

    let generics = add_trait_bounds(generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Attrs {
        collection_name,
        default_sorting_field,
        enable_nested_fields,
    } = extract_attrs(attrs)?;
    let collection_name = collection_name.unwrap_or_else(|| ident.to_string().to_lowercase());

    if let Some(ref sorting_field) = default_sorting_field
        && !fields.iter().any(|field|
                // At this point we are sure that this field is a named field.
                field.ident.as_ref().unwrap() == sorting_field)
    {
        return Err(syn::Error::new_spanned(
            item_ts,
            format!(
                "defined default_sorting_field = \"{sorting_field}\" does not match with any field."
            ),
        ));
    }

    let typesense_fields = fields
        .iter()
        .map(to_typesense_field_type)
        .collect::<syn::Result<Vec<_>>>()?;

    let default_sorting_field = if let Some(v) = default_sorting_field {
        quote! {
            let builder = builder.default_sorting_field(#v);
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    let enable_nested_fields = if let Some(v) = enable_nested_fields {
        quote! {
            let builder = builder.enable_nested_fields(#v);
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    let generated_code = quote! {
        impl #impl_generics typesense::prelude::Document for #ident #ty_generics #where_clause {
            const COLLECTION_NAME: &str = #collection_name;

            fn collection_schema() -> typesense::models::CollectionSchema {
                let name = Self::COLLECTION_NAME.to_owned();
                let fields = vec![#(#typesense_fields,)*];

                let builder = typesense::models::CollectionSchema::builder().name(name).fields(fields);

                #default_sorting_field
                #enable_nested_fields

                builder.build()
            }
        }
    };
    Ok(generated_code.into())
}

// Get the inner type for a given wrapper
fn ty_inner_type<'a>(ty: &'a syn::Type, wrapper: &'static str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(p) = ty
        && p.path.segments.len() == 1
        && p.path.segments[0].ident == wrapper
        && let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments
        && inner_ty.args.len() == 1
    {
        // len is 1 so this should not fail
        let inner_ty = inner_ty.args.first().unwrap();
        if let syn::GenericArgument::Type(t) = inner_ty {
            return Some(t);
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

#[derive(Default)]
struct Attrs {
    collection_name: Option<String>,
    default_sorting_field: Option<String>,
    enable_nested_fields: Option<bool>,
}

fn skip_eq(i: Ident, tt_iter: &mut impl Iterator<Item = TokenTree>) -> syn::Result<()> {
    match tt_iter.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == '=' => Ok(()),
        Some(tt) => Err(syn::Error::new_spanned(
            &tt,
            format!("Unexpected \"{tt}\", expected equal sign \"=\""),
        )),
        None => Err(syn::Error::new_spanned(i, "expected: equal sign \"=\"")),
    }
}

fn string_literal(tt_iter: &mut impl Iterator<Item = TokenTree>) -> syn::Result<String> {
    match tt_iter.next() {
        Some(TokenTree::Literal(l)) => {
            let lit = syn::Lit::new(l);
            if let syn::Lit::Str(s) = lit {
                Ok(s.value())
            } else {
                Err(syn::Error::new_spanned(
                    lit,
                    "it must be equal to a literal string",
                ))
            }
        }
        Some(TokenTree::Ident(i)) => Err(syn::Error::new(
            i.span(),
            format!("Expected string literal, did you mean \"{i}\"?"),
        )),
        tt => Err(syn::Error::new(tt.span(), "Expected string literal")),
    }
}

fn extract_attrs(attrs: Vec<Attribute>) -> syn::Result<Attrs> {
    let mut res = Attrs::default();

    let attr = match attrs
        .into_iter()
        .find(|a| a.path.segments.first().map(|s| s.ident == "typesense") == Some(true))
    {
        Some(a) => a,
        None => return Ok(res),
    };

    if let Some(TokenTree::Group(g)) = attr.tokens.into_iter().next() {
        let mut tt_iter = g.stream().into_iter();
        while let Some(tt) = tt_iter.next() {
            if let TokenTree::Ident(i) = tt {
                match &i.to_string() as &str {
                    "collection_name" => {
                        skip_eq(i, &mut tt_iter)?;
                        res.collection_name = Some(string_literal(&mut tt_iter)?);
                    }
                    "default_sorting_field" => {
                        skip_eq(i, &mut tt_iter)?;
                        res.default_sorting_field = Some(string_literal(&mut tt_iter)?);
                    }
                    "enable_nested_fields" => {
                        skip_eq(i, &mut tt_iter)?;
                        let val = match tt_iter.next() {
                            Some(TokenTree::Ident(i)) => &i.to_string() == "true",
                            tt => {
                                return Err(syn::Error::new(
                                    tt.span(),
                                    "Expected boolean, without quotation marks (\"\")",
                                ));
                            }
                        };
                        res.enable_nested_fields = Some(val);
                    }
                    v => {
                        return Err(syn::Error::new(i.span(), format!("Unexpected \"{v}\"")));
                    }
                }
            };
            if let Some(TokenTree::Punct(p)) = tt_iter.next() {
                let ch = p.as_char();
                if ch != ',' {
                    return Err(syn::Error::new(
                        p.span(),
                        format!("Unexpected \"{ch}\", expected comma \",\""),
                    ));
                }
            }
        }
    }

    Ok(res)
}

/// Convert a given field in a typesense field type.
fn to_typesense_field_type(field: &Field) -> syn::Result<proc_macro2::TokenStream> {
    let name = &field.ident;

    let facet = {
        let facet_vec = field
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path.segments.len() == 1
                    && attr.path.segments[0].ident == "typesense"
                    && let Some(proc_macro2::TokenTree::Group(g)) =
                        attr.tokens.clone().into_iter().next()
                {
                    let mut tokens = g.stream().into_iter();
                    match tokens.next() {
                        Some(proc_macro2::TokenTree::Ident(ref i)) => {
                            if i != "facet" {
                                return Some(Err(syn::Error::new_spanned(
                                    i,
                                    format!("Unexpected token {i}. Did you mean `facet`?"),
                                )));
                            }
                        }
                        Some(ref tt) => {
                            return Some(Err(syn::Error::new_spanned(
                                tt,
                                format!("Unexpected token {tt}. Did you mean `facet`?"),
                            )));
                        }
                        None => {
                            return Some(Err(syn::Error::new_spanned(attr, "expected `facet`")));
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
        <#ty as typesense::prelude::ToTypesenseField>::to_typesense_type().to_owned()
    );

    Ok(quote! {
        typesense::models::Field::builder().name(std::string::String::from(stringify!(#name))).r#type(#typesense_field_type)
            .maybe_optional(#optional)
            .maybe_facet(#facet)
            .build()
    })
}
