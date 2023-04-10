use std::{net::TcpStream, io::{Read, Write}};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
        <Title text="Rust Chat Application"/>

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

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    // connect to tcp stream
    let mut stream = TcpStream::connect("localhost:8081").unwrap();
    // read from stream and collect into variable
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // print out the buffer
    println!("Received: {}", String::from_utf8_lossy(&buffer[..]));
    
    // write to stream
    stream.write(b"Hello from the client!").unwrap();
    println!("Sent Hello, awaiting reply...");

    view! { cx,
        <h1>"Rust Chat Application"</h1>
        // show the received data from the server
        <p>{String::from_utf8_lossy(&buffer[..])}</p>
        <button on:click=on_click></button>
    }
}
