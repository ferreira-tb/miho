use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

pub fn impl_git(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
      impl Git for #name {
        async fn spawn(&mut self) -> anyhow::Result<std::process::ExitStatus> {
          let mut child = self.command.args(&self.args).spawn()?;
          let status = child.wait().await?;
          Ok(status)
        }
      }
  };

  gen.into()
}
