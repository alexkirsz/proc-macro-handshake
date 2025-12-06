use std::sync::Mutex;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    LitStr, Path, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

type Registry = Vec<(String, String)>;

static REGISTRY: Mutex<Registry> = Mutex::new(vec![]);

struct RegisterInput {
    registry: Path,
    _comma: Token![,],
    plugin: Path,
}

impl Parse for RegisterInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            // TODO: Validate that this is an absolute crate path (::some_crate).
            // This is the only way to ensure that further macro calls referring to this
            // crate won't call some shadowed binding instead.
            registry: input.parse()?,
            _comma: input.parse::<Token![,]>()?,
            plugin: input.parse()?,
        })
    }
}

/// Expands to `<plugin>::instantiate!(<registry_path>);`.
#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    let RegisterInput {
        registry,
        _comma,
        plugin,
    } = parse_macro_input!(input as RegisterInput);

    quote! {
        #plugin::instantiate!(#plugin, #registry);
    }
    .into()
}

struct RegisterPluginInput {
    path: Path,
    _comma: Token![,],
    init_data: LitStr,
}

impl Parse for RegisterPluginInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            _comma: input.parse()?,
            init_data: input.parse()?,
        })
    }
}

/// Records a plugin in the compile-time registry.
#[proc_macro]
pub fn register_plugin(input: TokenStream) -> TokenStream {
    let RegisterPluginInput {
        path,
        _comma,
        init_data,
    } = parse_macro_input!(input as RegisterPluginInput);

    REGISTRY
        .lock()
        .expect("registry mutex poisoned")
        // Can't reuse TokenStreams across invocations, so we go through a String and parse again
        // on the next call.
        .push((path.to_token_stream().to_string(), init_data.value()));

    TokenStream::new()
}

/// Expands to a vector containing all registered plugins, their init data, and the result of calling
/// a macro on them.
#[proc_macro]
pub fn list(_input: TokenStream) -> TokenStream {
    let entries = REGISTRY.lock().expect("registry mutex poisoned").clone();

    let items = entries.iter().map(|(path, data)| {
        let path_resolved = syn::parse_str::<Path>(path).expect("invalid path");

        quote! { (#path.to_string(), #data.to_string(), #path_resolved::data!().to_string()) }
    });

    quote! {
        Vec::<(String, String, String)>::from([ #(#items),* ])
    }
    .into()
}
