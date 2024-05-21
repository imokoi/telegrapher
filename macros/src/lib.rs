extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(BotCommands)]
pub fn bot_commands_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_bot_commands(&ast)
}

fn impl_bot_commands(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = match ast.data {
        syn::Data::Enum(ref data_enum) => {
            let variant_names = data_enum.variants.iter().map(|v| {
                let ident = &v.ident;
                let cmd_name = format!("/{}", ident.to_string().to_lowercase());
                quote! {
                    Commands::#ident => #cmd_name,
                }
            });

            quote! {
                impl #name {
                    pub fn command_name(&self) -> &'static str {
                        match self {
                            #( #variant_names )*
                        }
                    }
                }
            }
        }
        _ => panic!("#[derive(BotCommands)] is only defined for enums"),
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn my_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;
    let gen = quote! {
        fn #name() {
            println!("Function {} is called", stringify!(#name));
            #block
        }
    };
    gen.into()
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
