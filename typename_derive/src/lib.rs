use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// proc_macro_derive を付けた関数が derive マクロになる
#[proc_macro_derive(TypeName)]
pub fn derive_typename(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    // quote! は Rustのソースコードを受け取り、TokenStreamを生成する
    let gen = quote! {
        // typename がクレートであることを示している表現 (モジュールやエイリアスではない)
        impl ::typename::TypeNameTrait for #name {
            fn type_name(&self) -> &str {
                // #name は quote! の変数展開のための記法で、 name変数の中身に展開される
                stringify!(#name)
            }
        }
    };

    gen.into()
}
