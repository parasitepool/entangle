use leptos::prelude::*;
use leptos_meta::{Link, MetaTags, Title};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    hooks::use_location,
};

/// HTML shell wrapping the App. Used server-side to render the full document.
#[cfg(feature = "server")]
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
            <body style="margin: 0; font-family: system-ui, -apple-system, sans-serif; \
                         background: #0f0f23; color: #e0e0e0;">
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
        <Title text="entangle"/>
        <Link rel="icon" type_="image/svg+xml" href="/favicon.svg"/>
        <Router>
            <Layout>
                <Routes fallback=|| view! { <p>"Page not found"</p> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("listings") view=ListingsPage/>
                </Routes>
            </Layout>
        </Router>
    }
}

/// Page layout with top nav bar, slide-out sidebar, and main content area.
#[component]
fn Layout(children: Children) -> impl IntoView {
    let sidebar_open = RwSignal::new(false);

    view! {
        <NavBar sidebar_open/>
        <Sidebar sidebar_open/>
        <main style=move || format!(
            "margin-top: 48px; padding: 24px; \
             margin-left: {}px; transition: margin-left 0.2s ease;",
            if sidebar_open.get() { 240 } else { 0 }
        )>
            {children()}
        </main>
    }
}

/// Top navigation bar.
#[component]
fn NavBar(sidebar_open: RwSignal<bool>) -> impl IntoView {
    let toggle = move |_| sidebar_open.update(|open| *open = !*open);
    let location = use_location();
    let show_toggle = move || location.pathname.get() != "/" || sidebar_open.get();

    view! {
        <nav style="position: fixed; top: 0; left: 0; right: 0; height: 48px; \
                     background: #1a1a2e; color: #fff; display: flex; \
                     align-items: center; padding: 0 16px; z-index: 100; \
                     box-shadow: 0 2px 4px rgba(0,0,0,0.2);">
            <button
                on:click=toggle
                style=move || format!(
                    "background: none; border: none; color: #fff; \
                     font-size: 20px; cursor: pointer; margin-right: 16px; \
                     padding: 4px 8px; visibility: {};",
                    if show_toggle() { "visible" } else { "hidden" }
                )
            >
                {move || if sidebar_open.get() { "\u{2715}" } else { "\u{2630}" }}
            </button>
            <span style="font-weight: 700; font-size: 18px; margin-right: 32px;">
                "entangle"
            </span>
            <NavLink href="/" label="Home"/>
            <NavLink href="/listings" label="Listings"/>
        </nav>
    }
}

/// A navigation link that highlights when active.
#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    let location = use_location();
    let is_active = move || location.pathname.get() == href;

    view! {
        <a
            href=href
            style=move || format!(
                "text-decoration: none; padding: 4px 12px; font-size: 14px; color: {};",
                if is_active() { "#6c63ff" } else { "#ccc" }
            )
        >
            {label}
        </a>
    }
}

/// Slide-out sidebar panel.
#[component]
fn Sidebar(sidebar_open: RwSignal<bool>) -> impl IntoView {
    let style = move || {
        let translate = if sidebar_open.get() {
            "translateX(0%)"
        } else {
            "translateX(-100%)"
        };
        format!(
            "position: fixed; top: 48px; left: 0; bottom: 0; width: 240px; \
             background: #16213e; z-index: 90; \
             transition: transform 0.2s ease; \
             transform: {translate}; \
             box-shadow: 2px 0 8px rgba(0,0,0,0.15);"
        )
    };

    view! {
        <aside style=style>
            <div style="padding: 16px;"></div>
        </aside>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div style="text-align: center; padding-top: 80px;">
            <h1 style="font-size: 48px; margin: 0; color: #6c63ff;">"entangle"</h1>
            <p style="font-size: 20px; color: #f72585; margin-top: 8px;">"Swaps made easy"</p>
        </div>
    }
}

#[component]
fn ListingsPage() -> impl IntoView {
    view! {
        <h1 style="color: #6c63ff;">"Listings"</h1>
        <p>"No listings yet."</p>
    }
}
