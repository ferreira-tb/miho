use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

pub fn impl_git(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
      impl MihoCommand for #name {
        fn cmd(&mut self) -> &mut std::process::Command {
          &mut self.cmd
        }

        fn output(&mut self) -> anyhow::Result<std::process::Output> {
          let args = self.args.as_slice();
          let output = self.cmd.args(args).output()?;
          Ok(output)
        }

        fn spawn(&mut self) -> anyhow::Result<std::process::Child> {
          let args = self.args.as_slice();
          let child = self.cmd.args(args).spawn()?;
          Ok(child)
        }
      }
  };

  gen.into()
}
