extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DataEnum, DeriveInput};

#[proc_macro_derive(BotCommands)]
pub fn bot_commands_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_bot_commands(&ast).into()
}

fn impl_bot_commands(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let data_enum = get_enum_data(&input);

    let fn_command_name = impl_command_name(&data_enum);
    let fn_to_name_vec = impl_to_name_vec(&data_enum);
    let fn_try_into = impl_try_from(&data_enum);

    quote! {
        impl BotCommands for #name {
            #fn_command_name
            #fn_to_name_vec
        }

        impl std::convert::TryFrom<&str> for #name {
            type Error = &'static str;

            #fn_try_into
        }
    }
}

fn impl_command_name(data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let variant_names = data_enum.variants.iter().map(|v| {
        let ident = &v.ident;
        let cmd_name = format!("/{}", to_snake_case(&ident.to_string()));
        quote! {
            Commands::#ident => #cmd_name,
        }
    });

    quote! {
        fn command_name(&self) -> &'static str {
            match self {
                #( #variant_names )*
            }
        }
    }
    .into()
}

fn impl_to_name_vec(data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let variant_names = data_enum.variants.iter().map(|v| {
        let ident = &v.ident;
        let cmd_name = format!("/{}", to_snake_case(&ident.to_string()));
        quote! {
            #cmd_name,
        }
    });

    quote! {
        fn to_name_vec() -> Vec<&'static str> {
            vec![
                #( #variant_names )*
            ]
        }
    }
    .into()
}

fn impl_try_from(data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let variant_names = data_enum.variants.iter().map(|v| {
        let ident = &v.ident;
        let cmd_name = format!("/{}", to_snake_case(&ident.to_string()));
        println!("cmd_name: {}", cmd_name);
        quote! {
            #cmd_name => Ok(Commands::#ident),
        }
    });

    quote! {
        fn try_from(value: &str) -> Result<Self, &'static str> {
            match value {
                #( #variant_names )*
                _ => Err("Invalid command"),
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn event_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);

    // 获取函数名称
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let fn_inputs = &input_fn.sig.inputs;

    let output = quote! {
        pub fn #fn_name(#fn_inputs) -> Pin<Box<dyn Future<Output = TelegrapherResult<Option<JsonData>>> + Send>> {
            Box::pin(async move {
                #fn_body
            })
        }
    };

    output.into()
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

fn get_enum_data(input: &DeriveInput) -> syn::DataEnum {
    match input.data {
        syn::Data::Enum(ref data) => data.clone(),
        _ => panic!("#[derive(BotCommands)] is only defined for enums"),
    }
}

fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                snake_case.push('_');
            }
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}
