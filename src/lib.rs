use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemEnum, Type};

#[proc_macro_attribute]
pub fn aggregate(_attrs: TokenStream, body: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(body as ItemEnum);
    let original_code = ast.clone();

    // Uncomment this to inspect the ast of the original code.
    // eprintln!("{:#?}", ast);

    let outer_type = ast.ident;
    let variant_type_pairs: Vec<(Ident, Type)> = ast
        .variants
        .iter()
        .map(|variant| {
            (
                variant.ident.clone(),
                variant
                    .fields
                    .iter()
                    .next()
                    .expect("exactly one field per variant")
                    .ty
                    .clone(),
            )
        })
        .collect();
    let variants = variant_type_pairs
        .iter()
        .map(|(v, _t)| v)
        .collect::<Vec<_>>();
    let types = variant_type_pairs
        .iter()
        .map(|(_v, t)| t)
        .collect::<Vec<_>>();

    let output = quote! {
        // First keep the original code in tact
        #original_code

        // Now write all the From impls
        //TODO How does it know that `variants` and `types` are the same length? And what if they're not?
        #(
            impl From<#types> for #outer_type {
                fn from(b: #types) -> Self {
                    Self::#variants(b)
                }
            }
        )*
    };

    output.into()
}
