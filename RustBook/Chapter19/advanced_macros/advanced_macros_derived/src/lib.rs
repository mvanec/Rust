use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    let x = format!(
        r#"
        fn index() {{
            println!("entering");
            println!("args tokens: {{}}", {args});
            println!("input tokens: {{}}", {input});
            println!("exiting");
        }}
    "#,
        args = args.into_iter().count(),
        input = input.into_iter().count(),
    );

    x.parse().expect("Generated invalid tokens")

    // let args_token = syn::parse(args).unwrap();
    // let input_token = syn::parse(input).unwrap();

    // // Build the trait implementation
    // impl_route(&args_token, &input_token)
}

// fn impl_route(attr_token: &syn::DeriveInput, item_token: &syn::DeriveInput) -> TokenStream {
//     let attr_name = &attr_token.ident;
//     let body_name = &item_token.ident;

//     let gen = quote! {
//         impl route for #attr_name, #body_name {
//             fn index_macro() {
//                 let attrs = stringify!(#attr_name);
//                 let body = stringify!(#body_name);
//                 println!("Route = {}, with body =\n{}", attrs, body);
//             }
//         }
//     };
//     gen.into()
// }
