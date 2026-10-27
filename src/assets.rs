//! Assets are looked up by base name; see `build.rs` for how the table is made

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

/// URL for an asset under `assets/` by its path without extension
/// e.g. `asset("images/pfp")` -> `/assets/images/pfp.jpg`
pub fn asset(base: &str) -> Option<&'static str> {
    ASSETS.iter().find(|(name, _)| *name == base).map(|(_, url)| *url)
}

/// Like [`asset`], for icons in `assets/icons/`
pub fn icon(name: &str) -> Option<&'static str> {
    asset(&format!("icons/{name}"))
}
