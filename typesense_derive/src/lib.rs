mod field_attributes;
mod helpers;

use field_attributes::{extract_field_attrs, process_field};
use helpers::*;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{ToTokens, quote};
use syn::{Attribute, ItemStruct, spanned::Spanned};

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
        vis,
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
        let field_names_and_renames = fields
            .iter()
            .map(|field| {
                extract_field_attrs(field).map(|attrs| {
                    attrs
                        .rename
                        .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string())
                })
            })
            .collect::<syn::Result<Vec<String>>>()?;

        if !field_names_and_renames
            .iter()
            .any(|name| name == sorting_field)
        {
            return Err(syn::Error::new_spanned(
                item_ts,
                format!(
                    "defined default_sorting_field = \"{sorting_field}\" does not match with any field."
                ),
            ));
        }
    }

    let mut regular_fields = Vec::new();
    let mut flattened_fields = Vec::new();
    for field in &fields {
        let (regular, flattened) = process_field(field)?;
        if let Some(f) = regular {
            regular_fields.push(f);
        }
        if let Some(f) = flattened {
            flattened_fields.push(f);
        }
    }

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
            let builder = builder.symbols_to_index(vec![#(#v.to_owned()),*]);
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    let token_separators = if let Some(v) = token_separators {
        quote! {
            let builder = builder.token_separators(vec![#(#v.to_owned()),*]);
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    // Create Partial struct for document update
    let optional_fields = fields.iter().filter_map(|f| {
        let ident = f.ident.as_ref()?;
        if ident == "id" {
            return None;
        }
        let vis = &f.vis;
        let ty = &f.ty;

        Some(quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            #vis #ident: Option<#ty>,
        })
    });

    let name_partial = Ident::new(&(ident.to_string() + "Partial"), ident.span());

    let generated_code = quote! {
        #[derive(Default, Serialize)]
        #vis struct #name_partial {
            #(#optional_fields)*
        }
        impl ::typesense::prelude::DocumentPartial for #name_partial {}

        impl #impl_generics ::typesense::prelude::Document for #ident #ty_generics #where_clause {
            const COLLECTION_NAME: &str = #collection_name;

            type Partial = #name_partial;

            fn collection_schema() -> ::typesense::models::CollectionSchema<'static> {
                let fields = [#(#regular_fields,)*].into_iter()
                    #(.chain(#flattened_fields))*
                    .collect::<Vec<_>>();

                let builder = ::typesense::models::CollectionSchema::builder().name(Self::COLLECTION_NAME).fields(fields);

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
                .push(syn::parse_quote!(::typesense::field::ToTypesenseField));
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
