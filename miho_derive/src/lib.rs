mod commit;

use proc_macro::TokenStream;

#[proc_macro_derive(Commit)]
pub fn commit_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  commit::impl_commit(&ast)
}