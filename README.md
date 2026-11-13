# hadi.rocks

Personal site built with [Leptos](https://leptos.dev) (client-side rendered) and
[Trunk](https://trunkrs.dev), compiled to WebAssembly.

The previous Next.js version lives in `../portfolio-old-nextjs`.

## Run locally

Homebrew's `rust` shadows rustup and has no wasm target, so put rustup first:

```sh
export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
trunk serve        # http://127.0.0.1:8080
trunk build --release   # output in dist/
```

## Where things are

```text
src/content.rs       All text, links, education and projects (edit here)
src/languages.rs     Language breakdown, regenerated weekly (don't edit)
src/components/      Sidebar, README card, contact card, shared Card and Icon
src/assets.rs        asset("images/pfp") -> URL, resolved by base name
build.rs             Scans assets/ and builds that lookup table
style/main.css       Palette (CSS variables at the top) and layout
assets/images/       pfp, background (blurred behind the panel)
assets/icons/        Monochrome SVG icons, tinted by CSS
scripts/             update_languages.py, run by .github/workflows/languages.yml
```

## Swapping assets

Assets are referenced by base name, not file name. To replace the profile
picture, drop in `assets/images/pfp.png` (or `.jpg`, `.jpeg`, `.webp`) and
delete the old one. If several exist, the order is svg, webp, avif, png, jpg,
jpeg, gif. `trunk serve` picks up the change on its own.

Icons in `assets/icons/` are used as CSS masks, so any single-colour SVG works
and takes the surrounding text colour. Add one, then reference it by name in
`src/content.rs`.

## Language breakdown

`.github/workflows/languages.yml` runs every Monday (or by hand from the
Actions tab). It sums GitHub's per-language byte counts over the public,
non-fork, non-archived repos on x7lex, rewrites `src/languages.rs`, and commits
it if anything changed. Colours, the number of languages shown and excluded
repos are at the top of `scripts/update_languages.py`.
