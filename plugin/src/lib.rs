use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Path, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct InstantiateInput {
    self_path: Path,
    _comma: Token![,],
    registry_path: Path,
}

impl Parse for InstantiateInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            self_path: input.parse()?,
            _comma: input.parse()?,
            registry_path: input.parse()?,
        })
    }
}

/// Called by the registry's `register!` macro. Registers this plugin with the given registry path.
#[proc_macro]
pub fn instantiate(input: TokenStream) -> TokenStream {
    let InstantiateInput {
        self_path,
        _comma,
        registry_path,
    } = parse_macro_input!(input as InstantiateInput);

    quote! {
        #registry_path::register_plugin!(#self_path, "init data");
    }
    .into()
}

#[proc_macro]
pub fn data(_input: TokenStream) -> TokenStream {
    quote! {
        "call data"
    }
    .into()
}
