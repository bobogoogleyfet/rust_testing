use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <main>
            <h1>"Rust + Leptos + GitHub Pages"</h1>
            <p>"This is a high-performance Rust web app running for free."</p>
            <button
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "Click me: " {count}
            </button>
        </main>
    }
}
