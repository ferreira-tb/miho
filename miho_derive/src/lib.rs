mod git;
mod manifest;

use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(Manifest)]
pub fn manifest_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  manifest::impl_manifest(&ast)
}

#[proc_macro_derive(GitCommand)]
pub fn git_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  git::impl_git(&ast)
}
