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
src/content.rs       All text, links, education and tech stack (edit here)
src/components/      Sidebar, README card, contact card, shared Card and Icon
src/assets.rs        asset("images/pfp") -> URL, resolved by base name
build.rs             Scans assets/ and builds that lookup table
style/main.css       Palette (CSS variables at the top) and layout
assets/images/       pfp (background is kept but unused)
assets/icons/        Monochrome SVG icons, tinted by CSS
```

## Swapping assets

Assets are referenced by base name, not file name. To replace the profile
picture, drop in `assets/images/pfp.png` (or `.jpg`, `.jpeg`, `.webp`) and
delete the old one. If several exist, the order is svg, webp, avif, png, jpg,
jpeg, gif. `trunk serve` picks up the change on its own.

Icons in `assets/icons/` are used as CSS masks, so any single-colour SVG works
and takes the surrounding text colour. Add one, then reference it by name in
`src/content.rs`.

## Placeholders

The Signal and Book a meeting buttons point to `#` until real links are set in
`src/content.rs`.
