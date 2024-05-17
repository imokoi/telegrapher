extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

// #[macro_export]
// macro_rules! my_vec {
//     ($($x:expr), *) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_my_trait(&ast)
}

fn impl_my_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MyTrait for #name {
            fn hello(&self) {
                println!("Hello, I'm a {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

// #[derive(MyTrait)]
// struct MyStruct;

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
