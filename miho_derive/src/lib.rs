use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::path::Path;
use syn::{self, DeriveInput};

#[proc_macro_derive(Manifest)]
pub fn manifest_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_manifest(&ast)
}

fn impl_manifest(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let kebak = name.to_string().to_case(Case::Kebab);
  let filename = kebak.replace("-", ".");
  let ext = Path::new(&filename).extension().unwrap();
  let ext = ext.to_str().unwrap().to_lowercase();

  let parser = match ext.as_str() {
    "toml" => Ident::new("toml", Span::call_site()),
    "json" => Ident::new("serde_json", Span::call_site()),
    _ => panic!("invalid extension: {ext}"),
  };

  let gen = quote! {
      impl Manifest for #name {
        type Value = #parser::Value;

        fn read<P: AsRef<std::path::Path>>(path: P) -> Result<Box<dyn ManifestHandler>> {
          let contents = std::fs::read_to_string(path)?;
          let manifest: #name = #parser::from_str(&contents)?;
          Ok(Box::new(manifest))
        }

        fn read_as_value<P: AsRef<std::path::Path>>(path: P) -> Result<Self::Value> {
          let contents = std::fs::read_to_string(path)?;
          let manifest: Self::Value = #parser::from_str(&contents)?;
          Ok(manifest)
        }
      }
  };

  gen.into()
}

#[proc_macro_derive(Git)]
pub fn git_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_git(&ast)
}

fn impl_git(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
      impl MihoCommand for #name {
        fn cmd(&mut self) -> &mut Command {
          &mut self.cmd
        }

        fn output(&mut self) -> Result<Output> {
          let args = self.args.as_slice();
          let output = self.cmd.args(args).output()?;
          Ok(output)
        }

        fn spawn(&mut self) -> Result<Child> {
          let args = self.args.as_slice();
          let child = self.cmd.args(args).spawn()?;
          Ok(child)
        }
      }
  };

  gen.into()
}
