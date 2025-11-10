use crate::{bool_literal, get_inner_type, i32_literal, skip_eq, string_literal, ty_inner_type};
use proc_macro2::TokenTree;
use quote::quote;
use syn::{Attribute, Field};

#[derive(Default)]
pub(crate) struct FieldAttributes {
    type_override: Option<String>,
    facet: Option<bool>,
    index: Option<bool>,
    locale: Option<String>,
    sort: Option<bool>,
    infix: Option<bool>,
    num_dim: Option<i32>,
    optional: Option<bool>,
    store: Option<bool>,
    stem: Option<bool>,
    range_index: Option<bool>,
    vec_dist: Option<String>,
    flatten: bool,
    pub(crate) rename: Option<String>,
    skip: bool,
}

// This function will parse #[typesense(...)] on a FIELD
pub(crate) fn extract_field_attrs(field: &Field) -> syn::Result<FieldAttributes> {
    let attrs = &field.attrs;
    let mut res = FieldAttributes::default();

    // Find the single #[typesense] attribute, erroring if there are more than one.
    let all_ts_attrs: Vec<&Attribute> = attrs
        .iter()
        .filter(|a| a.path.get_ident().is_some_and(|i| i == "typesense"))
        .collect();

    //  Check for duplicates and create an error if found
    if all_ts_attrs.len() > 1 {
        return Err(syn::Error::new_spanned(
            field,
            "#[typesense(...)] is repeated more than one time.",
        ));
    }

    //  Get the single attribute, or return default if none exist
    let attr = if let Some(a) = all_ts_attrs.first() {
        *a
    } else {
        return Ok(res); // No typesense attribute, return default
    };

    if let Some(TokenTree::Group(g)) = attr.tokens.clone().into_iter().next() {
        let mut tt_iter = g.stream().into_iter().peekable();
        while let Some(tt) = tt_iter.next() {
            if let TokenTree::Ident(i) = tt {
                let is_shorthand =
                    tt_iter.peek().is_none() || tt_iter.peek().unwrap().to_string() == ",";
                let ident_str = i.to_string();

                match ident_str.as_str() {
                    // --- Boolean flags that support shorthand and key-value ---
                    "facet" | "sort" | "index" | "store" | "infix" | "stem" | "range_index"
                    | "optional" => {
                        let value = if is_shorthand {
                            true
                        } else {
                            skip_eq(&i, &mut tt_iter)?;
                            bool_literal(&mut tt_iter)?
                        };

                        // Set the correct field on the result struct, checking for duplicates
                        match ident_str.as_str() {
                            "facet" => {
                                if res.facet.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `facet` is duplicated",
                                    ));
                                }
                                res.facet = Some(value);
                            }
                            "sort" => {
                                if res.sort.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `sort` is duplicated",
                                    ));
                                }
                                res.sort = Some(value);
                            }
                            "index" => {
                                if res.index.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `index` is duplicated",
                                    ));
                                }
                                res.index = Some(value);
                            }
                            "store" => {
                                if res.store.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `store` is duplicated",
                                    ));
                                }
                                res.store = Some(value);
                            }
                            "infix" => {
                                if res.infix.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `infix` is duplicated",
                                    ));
                                }
                                res.infix = Some(value);
                            }
                            "stem" => {
                                if res.stem.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `stem` is duplicated",
                                    ));
                                }
                                res.stem = Some(value);
                            }
                            "range_index" => {
                                if res.range_index.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `range_index` is duplicated",
                                    ));
                                }
                                res.range_index = Some(value);
                            }
                            "optional" => {
                                if res.optional.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `optional` is duplicated",
                                    ));
                                }
                                res.optional = Some(value);
                            }
                            _ => unreachable!(),
                        }
                    }
                    // --- Flags that are ONLY shorthand ---
                    "flatten" | "skip" => {
                        if !is_shorthand {
                            return Err(syn::Error::new(
                                i.span(),
                                format!(
                                    "`{}` is a flag and does not take a value. Use `#[typesense({})]`",
                                    ident_str, ident_str
                                ),
                            ));
                        }
                        match ident_str.as_str() {
                            "flatten" => {
                                if res.flatten {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `flatten` is duplicated",
                                    ));
                                }
                                res.flatten = true;
                            }
                            "skip" => {
                                if res.skip {
                                    return Err(syn::Error::new_spanned(
                                        &i,
                                        "Attribute `skip` is duplicated",
                                    ));
                                }
                                res.skip = true;
                            }
                            _ => unreachable!(),
                        }
                    }

                    // --- Key-value only attributes ---
                    "rename" => {
                        skip_eq(&i, &mut tt_iter)?;
                        if res.rename.is_some() {
                            return Err(syn::Error::new_spanned(
                                &i,
                                "Attribute `rename` is duplicated",
                            ));
                        }
                        res.rename = Some(string_literal(&mut tt_iter)?);
                    }
                    "locale" => {
                        skip_eq(&i, &mut tt_iter)?;
                        if res.locale.is_some() {
                            return Err(syn::Error::new_spanned(
                                &i,
                                "Attribute `locale` is duplicated",
                            ));
                        }
                        res.locale = Some(string_literal(&mut tt_iter)?);
                    }
                    "vec_dist" => {
                        skip_eq(&i, &mut tt_iter)?;
                        if res.vec_dist.is_some() {
                            return Err(syn::Error::new_spanned(
                                &i,
                                "Attribute `vec_dist` is duplicated",
                            ));
                        }
                        res.vec_dist = Some(string_literal(&mut tt_iter)?);
                    }
                    "type" => {
                        skip_eq(&i, &mut tt_iter)?;
                        if res.type_override.is_some() {
                            return Err(syn::Error::new_spanned(
                                &i,
                                "Attribute `type` is duplicated",
                            ));
                        }
                        res.type_override = Some(string_literal(&mut tt_iter)?);
                    }
                    "num_dim" => {
                        skip_eq(&i, &mut tt_iter)?;
                        if res.num_dim.is_some() {
                            return Err(syn::Error::new_spanned(
                                &i,
                                "Attribute `num_dim` is duplicated",
                            ));
                        }
                        res.num_dim = Some(i32_literal(&mut tt_iter)?);
                    }
                    // --- Error for unknown attributes ---
                    v => {
                        return Err(syn::Error::new(
                            i.span(),
                            format!("Unexpected field attribute \"{}\"", v),
                        ));
                    }
                }
            };

            if let Some(TokenTree::Punct(p)) = tt_iter.peek()
                && p.as_char() == ','
            {
                tt_iter.next(); // Consume the comma
            }
        }
    }

    Ok(res)
}

fn build_regular_field(field: &Field, field_attrs: &FieldAttributes) -> proc_macro2::TokenStream {
    let (ty, is_option_type) = if let Some(inner_ty) = ty_inner_type(&field.ty, "Option") {
        (inner_ty, true)
    } else {
        (&field.ty, false)
    };

    let field_name = if let Some(rename) = &field_attrs.rename {
        quote! { #rename }
    } else {
        let name_ident = field.ident.as_ref().unwrap().to_string();
        quote! { #name_ident }
    };

    let typesense_field_type = if let Some(override_str) = &field_attrs.type_override {
        quote! { #override_str }
    } else {
        quote! { <#ty as ::typesense::prelude::ToTypesenseField>::to_typesense_type() }
    };

    let optional = field_attrs
        .optional
        .or(if is_option_type { Some(true) } else { None })
        .map(|v| quote!(.optional(#v)));
    let facet = field_attrs.facet.map(|v| quote!(.facet(#v)));
    let index = field_attrs.index.map(|v| quote!(.index(#v)));
    let store = field_attrs.store.map(|v| quote!(.store(#v)));
    let sort = field_attrs.sort.map(|v| quote!(.sort(#v)));
    let infix = field_attrs.infix.map(|v| quote!(.infix(#v)));
    let stem = field_attrs.stem.map(|v| quote!(.stem(#v)));
    let range_index = field_attrs.range_index.map(|v| quote!(.range_index(#v)));
    let locale = field_attrs.locale.as_ref().map(|v| quote!(.locale(#v)));
    let vec_dist = field_attrs.vec_dist.as_ref().map(|v| quote!(.vec_dist(#v)));
    let num_dim = field_attrs.num_dim.map(|v| quote!(.num_dim(#v)));

    quote! {
        ::typesense::models::Field::builder().name(#field_name).r#type(#typesense_field_type)
            #optional #facet #index #store #sort #infix #stem #range_index #locale #vec_dist #num_dim
            .build()
    }
}

/// Processes a single struct field.
/// Returns a TokenStream which evaluates to a `Vec<typesense::Field>`.
pub(crate) fn process_field(
    field: &Field,
) -> syn::Result<(
    Option<proc_macro2::TokenStream>,
    Option<proc_macro2::TokenStream>,
)> {
    let field_attrs = extract_field_attrs(field)?;

    if field_attrs.flatten {
        // Determine the prefix: use the rename value if it exists, otherwise use the field's name.
        let prefix = if let Some(rename_prefix) = &field_attrs.rename {
            quote! { #rename_prefix }
        } else {
            let name_ident = field.ident.as_ref().unwrap().to_string();
            quote! { #name_ident }
        };

        let inner_type = get_inner_type(&field.ty);
        let is_vec = ty_inner_type(&field.ty, "Vec").is_some()
            || ty_inner_type(&field.ty, "Option")
                .is_some_and(|t| ty_inner_type(t, "Vec").is_some());

        let flattened_fields = quote! {
            <#inner_type as ::typesense::prelude::Document>::collection_schema().fields
                .into_iter()
                .map(|mut f| {
                    // Use the dynamically determined prefix here
                    f.name = ::std::borrow::Cow::Owned(format!("{}.{}", #prefix, f.name));
                    if #is_vec && !f.r#type.ends_with("[]") {
                        f.r#type.to_mut().push_str("[]");
                    }
                    f
                })
        };

        if field_attrs.skip {
            // `#[typesense(flatten, skip)]` -> Only flattened fields
            return Ok((None, Some(quote! { #flattened_fields })));
        }

        // `#[typesense(flatten)]` -> Flattened fields + object field
        let regular_field = build_regular_field(field, &field_attrs);

        Ok((
            Some(quote! { #regular_field }),
            Some(quote! { #flattened_fields }),
        ))
    } else {
        // --- REGULAR FIELD LOGIC ---
        if field_attrs.skip {
            return Ok((None, None));
        }
        Ok((Some(build_regular_field(field, &field_attrs)), None))
    }
}
