use darling::{FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn};

#[derive(FromMeta)]
struct AocMacroArgs {
    test: String,
    part: Option<u8>
}

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(darling::Error::from(e).write_errors()); }
    };
    let args = match AocMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };
    let item = parse_macro_input!(item as ItemFn);
    let res = try_aoc(args, item).unwrap_or_else(|e| e.into_compile_error());
    res.into()
}

fn try_aoc(args: AocMacroArgs, item: ItemFn) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = item.sig.ident.clone();
    let test_value = args.test;
    let test_ident = format_ident!("{}_tests", ident);
    let test_fn_ident = format_ident!("test_{}", ident);
    let mut test = "../test.txt".to_string();
    if args.part.is_some(){
        test = format!("../test-{}.txt", args.part.unwrap());
    }
    Ok(
        quote!(
            #item

            #[cfg(test)]
            mod #test_ident {
                use super::*;

                #[test]
                fn #test_fn_ident() {
                    assert_eq!(#ident (include_str!(#test)).to_string(), #test_value)
                }
            }
        )
    )
} 