//! "Book a meeting" modal: blurs the page, shows a Cloudflare Turnstile check,
//! then sends the visitor to the booking page, with a manual link as a fallback.

use leptos::{ev, html, prelude::*};
use wasm_bindgen::prelude::*;

/// Public site key from `.env` (see `build.rs`). Without it the modal skips the
/// check and just shows the link.
const SITE_KEY: Option<&str> = option_env!("TURNSTILE_SITE_KEY");

#[wasm_bindgen(inline_js = r#"
export function renderTurnstile(el, sitekey, onSuccess, onError) {
  const render = () => {
    if (!el.isConnected) return;
    el.dataset.widgetId = window.turnstile.render(el, {
      sitekey,
      theme: "dark",
      callback: onSuccess,
      "error-callback": () => { onError(); return true; },
    });
  };
  if (window.turnstile) return render();
  // The api.js script loads async; wait for it.
  const wait = setInterval(() => {
    if (window.turnstile) { clearInterval(wait); render(); }
  }, 100);
}

export function removeTurnstile(el) {
  if (window.turnstile && el.dataset.widgetId) window.turnstile.remove(el.dataset.widgetId);
}
"#)]
extern "C" {
    #[wasm_bindgen(js_name = renderTurnstile)]
    fn render_turnstile(
        el: &web_sys::HtmlElement,
        sitekey: &str,
        on_success: &JsValue,
        on_error: &JsValue,
    );
    #[wasm_bindgen(js_name = removeTurnstile)]
    fn remove_turnstile(el: &web_sys::HtmlElement);
}

#[component]
pub fn BookingModal(open: RwSignal<bool>, href: &'static str) -> impl IntoView {
    let verified = RwSignal::new(SITE_KEY.is_none());
    let failed = RwSignal::new(false);
    let widget = NodeRef::<html::Div>::new();

    let close = move || {
        if let Some(el) = widget.get_untracked() {
            remove_turnstile(&el);
        }
        open.set(false);
        verified.set(SITE_KEY.is_none());
        failed.set(false);
    };

    widget.on_load(move |el| {
        let Some(key) = SITE_KEY else { return };
        let on_success = Closure::<dyn Fn(String)>::new(move |_token: String| {
            verified.set(true);
            let _ = window().location().set_href(href);
        });
        let on_error = Closure::<dyn Fn()>::new(move || failed.set(true));
        render_turnstile(&el, key, &on_success.into_js_value(), &on_error.into_js_value());
    });

    let _esc = window_event_listener(ev::keydown, move |e| {
        if e.key() == "Escape" && open.get_untracked() {
            close();
        }
    });

    view! {
        <div class="modal-backdrop" on:click=move |_| close()>
            <div
                class="modal"
                role="dialog"
                aria-modal="true"
                aria-labelledby="booking-title"
                on:click=|e| e.stop_propagation()
            >
                <div class="modal-head">
                    <h2 id="booking-title">"Book a meeting"</h2>
                    <button class="modal-close" aria-label="Close" on:click=move |_| close()>"×"</button>
                </div>
                <Show
                    when=move || verified.get()
                    fallback=move || view! {
                        <p class="muted">"Quick check that you're human, then you'll go to my calendar."</p>
                        <div class="turnstile" class:failed=move || failed.get() node_ref=widget></div>
                        <Show when=move || failed.get()>
                            <p class="muted">"The check couldn't load. You can still continue below."</p>
                            <a class="button" href=href>"Continue to cal.com"</a>
                        </Show>
                    }
                >
                    <p class="muted">"Redirecting to cal.com…"</p>
                    <a class="button primary" href=href>"Not redirected? Continue to cal.com"</a>
                </Show>
            </div>
        </div>
    }
}
