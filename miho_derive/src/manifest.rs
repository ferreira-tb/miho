use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::path::Path;
use syn::{self, DeriveInput};

pub fn impl_manifest(ast: &DeriveInput) -> TokenStream {
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

        fn read<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Box<dyn ManifestHandler>> {
          let contents = std::fs::read_to_string(path)?;
          let manifest: #name = #parser::from_str(&contents)?;
          Ok(Box::new(manifest))
        }

        fn read_as_value<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self::Value> {
          let contents = std::fs::read_to_string(path)?;
          let manifest: Self::Value = #parser::from_str(&contents)?;
          Ok(manifest)
        }
      }
  };

  gen.into()
}
