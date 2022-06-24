extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn);
    let identifier = &ast.sig.ident;

    let block = ast.block.as_ref();
    let block = parse_quote! {{
        println!("Enter: {}", stringify!(#identifier));

        let block = || #block;
        let result = block();

        println!("Exit: {}", stringify!(#identifier));

        result
    }};

    *ast.block = block;

    ast.into_token_stream().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
