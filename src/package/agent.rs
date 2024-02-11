/// Agent responsible for the manifest.
/// This tipically represents the package manager used.
pub enum Agent {
  Cargo,
  Npm,
  Pnpm,
  Tauri,
  Yarn,
}