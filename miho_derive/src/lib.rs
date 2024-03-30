#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]

mod commit;
mod git;

use proc_macro::TokenStream;

#[proc_macro_derive(Commit)]
pub fn commit_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  commit::impl_commit(&ast)
}

#[proc_macro_derive(Git)]
pub fn git_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  git::impl_git(&ast)
}
