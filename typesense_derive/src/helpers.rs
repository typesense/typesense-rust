use proc_macro2::{Ident, TokenTree};
use syn::spanned::Spanned;

// Helper to parse a boolean literal
pub(crate) fn bool_literal(tt_iter: &mut impl Iterator<Item = TokenTree>) -> syn::Result<bool> {
    match tt_iter.next() {
        Some(TokenTree::Ident(i)) => {
            if i == "true" {
                Ok(true)
            } else if i == "false" {
                Ok(false)
            } else {
                Err(syn::Error::new_spanned(
                    i,
                    "Expected a boolean `true` or `false`",
                ))
            }
        }
        tt => Err(syn::Error::new(tt.span(), "Expected a boolean literal")),
    }
}

// Helper to parse an integer literal
pub(crate) fn i32_literal(tt_iter: &mut impl Iterator<Item = TokenTree>) -> syn::Result<i32> {
    match tt_iter.next() {
        Some(TokenTree::Literal(l)) => {
            let lit = syn::Lit::new(l);
            if let syn::Lit::Int(i) = lit {
                i.base10_parse::<i32>()
            } else {
                Err(syn::Error::new_spanned(
                    lit,
                    "it must be equal to an integer literal",
                ))
            }
        }
        tt => Err(syn::Error::new(tt.span(), "Expected an integer literal")),
    }
}

pub(crate) fn string_literal(tt_iter: &mut impl Iterator<Item = TokenTree>) -> syn::Result<String> {
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

// Helper function to parse a bracketed list of string literals
pub(crate) fn string_list(
    tt_iter: &mut impl Iterator<Item = TokenTree>,
) -> syn::Result<Vec<String>> {
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
        if let Some(TokenTree::Punct(p)) = inner_iter.peek()
            && p.as_char() == ','
        {
            inner_iter.next(); // Consume the comma
        }
    }

    Ok(result)
}

pub(crate) fn skip_eq(i: &Ident, tt_iter: &mut impl Iterator<Item = TokenTree>) -> syn::Result<()> {
    match tt_iter.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == '=' => Ok(()),
        Some(tt) => Err(syn::Error::new_spanned(
            &tt,
            format!("Unexpected \"{tt}\", expected equal sign \"=\""),
        )),
        None => Err(syn::Error::new_spanned(i, "expected: equal sign \"=\"")),
    }
}

// Get the inner type for a given wrappper
pub(crate) fn ty_inner_type<'a>(ty: &'a syn::Type, wrapper: &'static str) -> Option<&'a syn::Type> {
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

/// Helper to get the inner-most type from nested Option/Vec wrappers.
pub(crate) fn get_inner_type(mut ty: &syn::Type) -> &syn::Type {
    while let Some(inner) = ty_inner_type(ty, "Option").or_else(|| ty_inner_type(ty, "Vec")) {
        ty = inner;
    }
    ty
}
