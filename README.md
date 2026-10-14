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
assets/images/       pfp, background (blurred behind the panel)
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

## Book a meeting (Turnstile)

Links with `gated: true` in `src/content.rs` open a modal that blurs the page,
runs a Cloudflare Turnstile check, then redirects. A manual link appears once
the check passes, or if the widget fails to load.

The site key is read from `SITEKEY` in `.env` (or the environment) at build
time. `SECRETKEY` is never read by the build, so it never reaches the browser.
This is a client-side check only: verifying the token with the secret needs a
server, which this static site doesn't have yet.

Turnstile only runs on hostnames allowed for the site key in the Cloudflare
dashboard. On 127.0.0.1 it shows error 110200 unless you add it there, or
test with Cloudflare's always-pass key:

```sh
SITEKEY=1x00000000000000000000AA trunk serve
```
