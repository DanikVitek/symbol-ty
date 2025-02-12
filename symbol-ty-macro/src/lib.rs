use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::FoundCrate;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, parse_quote_spanned, Error, LitStr, Path, Type};

const CRATE_NAME: &str = "symbol-ty";

/// Use this macro to reference the type, that corresponds to the given symbol
/// # Example
/// ```rust
/// # use symbol_ty::Symbol;
/// let s = <Symbol!("foo")>::new();
/// 
/// assert_eq!(s.to_string(), "foo");
/// assert_eq!(s, <Symbol!("foo")>::new());
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn Symbol(input: TokenStream) -> TokenStream {
    let cratename: Path = match proc_macro_crate::crate_name(CRATE_NAME) {
        Err(err) => return Error::new(Span::call_site(), err).to_compile_error().into(),
        Ok(FoundCrate::Itself) => parse_quote!(crate),
        Ok(FoundCrate::Name(cratename)) => {
            let ident = syn::Ident::new(&cratename, Span::call_site());
            parse_quote!(::#ident)
        }
    };

    parse_macro_input!(input as LitStr)
        .value()
        .chars()
        .rfold(
            parse_quote_spanned!(Span::mixed_site() => #cratename::Nil),
            |ty: Type, ch| parse_quote_spanned!(Span::mixed_site() => #cratename::Cons<#ch, #ty>),
        )
        .into_token_stream()
        .into()
}
