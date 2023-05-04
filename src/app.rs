use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use web_sys::WebSocket;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use js_sys::JsString;


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
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    // Create a signal to store and display WebSocket messages
    let (message, set_message) = create_signal(cx, "".to_string());

    // Connect to the WebSocket server
    let ws = WebSocket::new("ws://localhost:8080").unwrap();
    let ws_clone = ws.clone();
    
    let on_message_callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
        if let Ok(data) = event.data().dyn_into::<js_sys::JsString>() {
            let data_str: String = data.into();
            set_message(data_str);
        }
    }) as Box<dyn FnMut(_)>);
    
    ws_clone.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
    on_message_callback.forget();

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <p>{ "WebSocket Message: " } {message}</p>
    }
}