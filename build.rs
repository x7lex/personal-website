//! Scans `assets/` and maps each file's base name (path without extension) to
//! the file that actually exists, so code asks for "images/background" and gets
//! background.png, .jpg, .jpeg or .webp, whichever is present.
//!
//! It also passes the public Turnstile site key from `.env` (`SITEKEY`) to the
//! app. Only the site key is read; the secret key never ends up in the bundle.

use std::{env, fs, path::Path};

/// When several files share a base name, the earliest extension here wins.
const PREFERENCE: &[&str] = &["svg", "webp", "avif", "png", "jpg", "jpeg", "gif"];

fn main() {
    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-env-changed=SITEKEY");

    if let Some(key) = env::var("SITEKEY").ok().or_else(|| dotenv_value("SITEKEY")) {
        println!("cargo:rustc-env=TURNSTILE_SITE_KEY={key}");
    }

    let mut entries: Vec<(String, String)> = Vec::new();
    collect(Path::new("assets"), &mut entries);

    entries.sort_by(|(a, a_ext), (b, b_ext)| a.cmp(b).then(rank(a_ext).cmp(&rank(b_ext))));
    entries.dedup_by(|later, earlier| later.0 == earlier.0);

    let mut out = String::from("pub static ASSETS: &[(&str, &str)] = &[\n");
    for (base, ext) in &entries {
        out.push_str(&format!("    ({base:?}, \"/assets/{base}.{ext}\"),\n"));
    }
    out.push_str("];\n");

    let dest = Path::new(&env::var("OUT_DIR").unwrap()).join("assets.rs");
    fs::write(dest, out).unwrap();
}

fn rank(ext: &str) -> usize {
    PREFERENCE.iter().position(|p| *p == ext).unwrap_or(PREFERENCE.len())
}

fn collect(dir: &Path, entries: &mut Vec<(String, String)>) {
    let Ok(read) = fs::read_dir(dir) else { return };
    for entry in read.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, entries);
            continue;
        }
        let (Some(stem), Some(ext)) = (path.file_stem(), path.extension()) else { continue };
        let ext = ext.to_string_lossy().to_lowercase();
        if stem.to_string_lossy().starts_with('.') || rank(&ext) == PREFERENCE.len() {
            continue;
        }
        let rel = path.with_extension("");
        let base = rel.strip_prefix("assets").unwrap().to_string_lossy().replace('\\', "/");
        entries.push((base, ext));
    }
}

/// Reads one `KEY=value` line from `.env`, ignoring the rest of the file.
fn dotenv_value(name: &str) -> Option<String> {
    let file = fs::read_to_string(".env").ok()?;
    file.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        (key.trim() == name).then(|| value.trim().trim_matches(['"', '\'']).to_string())
    })
}
