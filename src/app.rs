use leptos::prelude::*;
use leptos_meta::MetaTags;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

/// HTML shell wrapping the App. Used server-side to render the full document.
#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// Root application component, shared between server and client.
#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <Router>
            <main>
                <Routes fallback=|| view! { <p>"Page not found"</p> }>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"entangle"</h1>
        <p>"Swaps made easy"</p>
    }
}
