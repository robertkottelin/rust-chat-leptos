

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::WebSocket;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let ws = WebSocket::new("ws://localhost:8080").unwrap();

    let (chat_messages, set_chat_messages) = create_signal(cx, vec![]);

    let onmessage_callback = Closure::wrap(Box::new(move |msg: JsValue| {
        let msg = msg.dyn_into::<web_sys::MessageEvent>().unwrap();
        let message = msg.data().as_string().unwrap_throw();
        set_chat_messages.update(|chat_messages| chat_messages.push(message));
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let on_send = move |text: String| {
        ws.send_with_str(&text).unwrap();
    };

    view! {
        cx,
        <div>
            <form
                on:submit=move |ev| {
                    ev.prevent_default();
                    let input = ev.target().unwrap().dyn_into::<web_sys::HtmlFormElement>().unwrap();
                    let input_value = input.query_selector("#message-input").unwrap().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
                    on_send(input_value.clone());
                    input.reset();
                }
            >
                <label>
                    "Message"
                    <input type="text" name="message" id="message-input"/>
                </label>
                <input type="submit" value="Send"/>
            </form>
            <ul>
                {chat_messages.get().iter().map(|msg| view! { cx, <li>{msg}</li> }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
