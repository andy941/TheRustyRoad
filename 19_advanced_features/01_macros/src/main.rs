// Declarative macro: simplified example of the `vec!` macro
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Procedural macro
/* use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
} */

// Custome derive macro: works only for structs and enums
/* #[proc_macro_derive(HelloMacro)]
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
} */

// Attribute-like macros: more flexible, allows to create new attributes
// #[proc_macro_attribute]
/* #[route(GET, "/")] // <- macro to annotate functions
fn index() {
    // snip
} */

// Function-like macros
// #[proc_macro]
/* let sql = sql!(SELECT * FROM posts WHERE id=1); */

fn main() {
}

