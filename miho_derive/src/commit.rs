use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

pub fn impl_commit(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
    impl CommitFromCommand for #name {
      async fn commit(&mut self, default_message: &str) -> anyhow::Result<()> {
        use miho::git::Git;

        if let Some(pathspec) = &self.add {
          miho::git::Add::new(pathspec).spawn().await?;
        }

        let message = if !self.no_ask && self.commit_message.is_none() {
          inquire::Text::new("Commit message: ").prompt_skippable()?
        } else {
          self.commit_message.take()
        };

        let message = match message.as_deref().map(str::trim) {
          Some(m) if !m.is_empty() => m,
          _ => default_message,
        };

        let mut commit = miho::git::Commit::new(message);
        commit.all();

        if self.no_verify {
          commit.no_verify();
        }

        commit.spawn().await?;

        if !self.no_push {
          miho::git::Push::new().spawn().await?;
        }

        Ok(())
      }
    }
  };

  gen.into()
}
