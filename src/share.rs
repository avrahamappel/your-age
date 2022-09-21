use gloo_utils::window;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::ShareData;
use yew::prelude::*;

enum Share {
    Facebook,
    Twitter,
    Pinterest,
    LinkedIn,
    Reddit,
    WhatsApp,
    Email,
}

impl Share {
    fn url(&self, url: &str, title: &str) -> String {
        match self {
            Share::Facebook => format!("https://www.facebook.com/sharer.php?u={}", title),
            Share::Twitter => format!("https://twitter.com/share?url={}&text={}", url, title),
            Share::Pinterest => format!(
                "https://pinterest.com/pin/create/bookmarklet/?url={}&description={}",
                url, title
            ),
            Share::LinkedIn => format!(
                "https://www.linkedin.com/shareArticle?url={}&title={}",
                url, title
            ),
            Share::Reddit => format!("https://reddit.com/submit?url={}&title={}", url, title),
            Share::Email => format!("mailto:?subject={}&body={}", url, title),
            Share::WhatsApp => format!("https://api.whatsapp.com/send?text={}+{}", url, title),
        }
    }
}

#[function_component(ShareButton)]
pub fn share_button() -> Html {
    let nav = window().navigator();
    let url = window()
        .location()
        .href()
        .expect_throw("Couldn't find current URL");
    let title = "Check out my age on YourAge!".replace(' ', "+");

    if nav.can_share() {
        let onclick = Callback::from(move |_| {
            nav.share_with_data(ShareData::new().url(&url).title(&title));
        });
        html! {
            <button {onclick}>{ "Share" }</button>
        }
    } else {
        html! {
            <div style="display:flex;">
                <a type="button" target="_blank" href={Share::Facebook.url(&url, &title)}>{ "Facebook" }</a>
                <a type="button" target="_blank" href={Share::Twitter.url(&url, &title)}>{ "Twitter" }</a>
                <a type="button" target="_blank" href={Share::Pinterest.url(&url, &title)}>{ "Pinterest" }</a>
                <a type="button" target="_blank" href={Share::LinkedIn.url(&url, &title)}>{ "LinkedIn" }</a>
                <a type="button" target="_blank" href={Share::Reddit.url(&url, &title)}>{ "Reddit" }</a>
                <a type="button" target="_blank" href={Share::WhatsApp.url(&url, &title)}>{ "WhatsApp" }</a>
                <a type="button" target="_blank" href={Share::Email.url(&url, &title)}>{ "Email" }</a>
            </div>
        }
    }
}
