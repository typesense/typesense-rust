use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{ToTokens, quote};
use syn::{Attribute, ItemStruct, spanned::Spanned};
mod field_attrs;
use field_attrs::process_field;

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
        symbols_to_index,
        token_separators,
    } = extract_attrs(attrs)?;
    let collection_name = collection_name.unwrap_or_else(|| ident.to_string().to_lowercase());

    if let Some(ref sorting_field) = default_sorting_field {
        if !fields.iter().any(|field|
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
    }

    //  Use flat_map to handle fields that expand into multiple schema fields
    let typesense_fields = fields
        .iter()
        .map(|field| process_field(field)) // process_field returns a Result<TokenStream>
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

    let symbols_to_index = if let Some(v) = symbols_to_index {
        quote! {
            let builder = builder.symbols_to_index(vec![#(#v.to_string()),*]);
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    let token_separators = if let Some(v) = token_separators {
        quote! {
            let builder = builder.token_separators(vec![#(#v.to_string()),*]);
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    let generated_code = quote! {
        impl #impl_generics typesense::prelude::Document for #ident #ty_generics #where_clause {
            fn collection_schema() -> typesense::models::CollectionSchema {
                let name = #collection_name.to_owned();

                // Collect fields from all sources
                let fields: Vec<typesense::Field> = vec![
                    #(#typesense_fields,)*
                ].into_iter().flatten().collect();

                // start the bon builder and set fields
                let builder = typesense::builders::new_collection_schema(name, fields);

                #default_sorting_field
                #enable_nested_fields
                #token_separators
                #symbols_to_index

                builder.build()
            }
        }
    };
    Ok(generated_code.into())
}

// Add a bound `T: ToTypesenseField` to every type parameter T.
fn add_trait_bounds(mut generics: syn::Generics) -> syn::Generics {
    for param in &mut generics.params {
        if let syn::GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(syn::parse_quote!(typesense::prelude::ToTypesenseField));
        }
    }
    generics
}

#[derive(Default)]
struct Attrs {
    collection_name: Option<String>,
    default_sorting_field: Option<String>,
    symbols_to_index: Option<Vec<String>>,
    enable_nested_fields: Option<bool>,
    token_separators: Option<Vec<String>>,
}

fn skip_eq(i: &Ident, tt_iter: &mut impl Iterator<Item = TokenTree>) -> syn::Result<()> {
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
                        skip_eq(&i, &mut tt_iter)?;
                        res.collection_name = Some(string_literal(&mut tt_iter)?);
                    }
                    "default_sorting_field" => {
                        skip_eq(&i, &mut tt_iter)?;
                        res.default_sorting_field = Some(string_literal(&mut tt_iter)?);
                    }
                    "enable_nested_fields" => {
                        skip_eq(&i, &mut tt_iter)?;
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
                    "symbols_to_index" => {
                        skip_eq(&i, &mut tt_iter)?;
                        res.symbols_to_index = Some(string_list(&mut tt_iter)?);
                    }
                    "token_separators" => {
                        skip_eq(&i, &mut tt_iter)?;
                        res.token_separators = Some(string_list(&mut tt_iter)?);
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

// Helper function to parse a bracketed list of string literals
fn string_list(tt_iter: &mut impl Iterator<Item = TokenTree>) -> syn::Result<Vec<String>> {
    let group = match tt_iter.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == proc_macro2::Delimiter::Bracket => g,
        Some(tt) => {
            return Err(syn::Error::new_spanned(
                tt,
                "Expected a list in brackets `[]`",
            ));
        }
        None => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected a list in brackets `[]`",
            ));
        }
    };

    let mut result = Vec::new();
    let mut inner_iter = group.stream().into_iter().peekable();

    while let Some(tt) = inner_iter.next() {
        if let TokenTree::Literal(l) = tt {
            let lit = syn::Lit::new(l);
            if let syn::Lit::Str(s) = lit {
                result.push(s.value());
            } else {
                return Err(syn::Error::new_spanned(lit, "Expected a string literal"));
            }
        } else {
            return Err(syn::Error::new_spanned(tt, "Expected a string literal"));
        }

        // Check for a trailing comma
        if let Some(TokenTree::Punct(p)) = inner_iter.peek() {
            if p.as_char() == ',' {
                inner_iter.next(); // Consume the comma
            }
        }
    }

    Ok(result)
}
