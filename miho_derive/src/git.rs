use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

pub fn impl_git(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
      impl Git for #name {
        fn stderr(&mut self, cfg: std::process::Stdio) -> &mut Self {
          self.command.stderr(cfg);
          self
        }

        fn stdout(&mut self, cfg: std::process::Stdio) -> &mut Self {
          self.command.stdout(cfg);
          self
        }

        async fn spawn(&mut self) -> anyhow::Result<std::process::ExitStatus> {
          let mut child = self.command.args(&self.args).spawn()?;
          let status = child.wait().await?;
          Ok(status)
        }

        async fn output(&mut self) -> anyhow::Result<std::process::Output> {
          let output = self.command.args(&self.args).output().await?;
          Ok(output)
        }
      }
  };

  gen.into()
}
