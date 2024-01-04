use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

pub fn impl_git(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
      impl #name {
        pub fn cmd(&mut self) -> &mut std::process::Command {
          &mut self.cmd
        }

        pub fn stderr(&mut self, cfg: std::process::Stdio) -> &mut Self {
          self.cmd().stderr(cfg);
          self
        }

        pub fn stdout(&mut self, cfg: std::process::Stdio) -> &mut Self {
          self.cmd().stdout(cfg);
          self
        }

        pub fn output(&mut self) -> anyhow::Result<std::process::Output> {
          let args = self.args.as_slice();
          let output = self.cmd.args(args).output()?;
          Ok(output)
        }

        pub fn spawn(&mut self) -> anyhow::Result<std::process::Child> {
          let args = self.args.as_slice();
          let child = self.cmd.args(args).spawn()?;
          Ok(child)
        }
      }
  };

  gen.into()
}
