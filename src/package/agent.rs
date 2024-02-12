/// Agent responsible for the manifest.
///
/// This tipically represents the package manager used.
#[derive(Clone, Debug)]
pub enum Agent {
  Cargo,
  Npm,
  Pnpm,
  Tauri,
  Yarn,
}
