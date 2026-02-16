use leptos::prelude::*;
#[cfg(feature = "server")]
use leptos_meta::MetaTags;
use leptos_meta::{Link, Title};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    hooks::use_location,
};

use crate::listing::{CreateListing, Listing, list_listings};

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
    let create_action = ServerAction::<CreateListing>::new();
    provide_context(create_action.version());

    view! {
        <NavBar sidebar_open/>
        <Sidebar sidebar_open create_action/>
        <main style=move || format!(
            "margin-top: 48px; padding: 24px; \
             margin-left: {}px; transition: margin-left 0.2s ease;",
            if sidebar_open.get() { 280 } else { 0 }
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

const INPUT_STYLE: &str = "width: 100%; padding: 6px 8px; background: #1a1a2e; \
    border: 1px solid #333; color: #e0e0e0; border-radius: 4px; \
    font-size: 13px; box-sizing: border-box;";

const LABEL_STYLE: &str = "display: block; font-size: 11px; color: #999; \
    margin: 8px 0 2px 0;";

/// Slide-out sidebar panel with listing creation form.
#[component]
fn Sidebar(
    sidebar_open: RwSignal<bool>,
    create_action: ServerAction<CreateListing>,
) -> impl IntoView {
    let pending = create_action.pending();
    let value = create_action.value();

    let style = move || {
        let translate = if sidebar_open.get() {
            "translateX(0%)"
        } else {
            "translateX(-100%)"
        };
        format!(
            "position: fixed; top: 48px; left: 0; bottom: 0; width: 280px; \
             background: #16213e; z-index: 90; \
             transition: transform 0.2s ease; \
             transform: {translate}; \
             box-shadow: 2px 0 8px rgba(0,0,0,0.15); \
             overflow-y: auto;"
        )
    };

    view! {
        <aside style=style>
            <div style="padding: 16px;">
                <h3 style="color: #6c63ff; margin: 0 0 12px 0; font-size: 14px;">
                    "New Listing"
                </h3>
                <ActionForm action=create_action>
                    <label style=LABEL_STYLE>"UTXO A *"</label>
                    <input type="text" name="utxo_a" required style=INPUT_STYLE
                        placeholder="txid:vout"/>

                    <label style=LABEL_STYLE>"Address A *"</label>
                    <input type="text" name="address_a" required style=INPUT_STYLE
                        placeholder="tb1q..."/>

                    <label style=LABEL_STYLE>"UTXO B"</label>
                    <input type="text" name="utxo_b" style=INPUT_STYLE
                        placeholder="txid:vout (optional)"/>

                    <label style=LABEL_STYLE>"Address B *"</label>
                    <input type="text" name="address_b" required style=INPUT_STYLE
                        placeholder="tb1q..."/>

                    <label style=LABEL_STYLE>"Network *"</label>
                    <select name="network" required style=INPUT_STYLE>
                        <option value="testnet">"Testnet"</option>
                        <option value="mainnet">"Mainnet"</option>
                        <option value="signet">"Signet"</option>
                    </select>

                    <label style=LABEL_STYLE>"Tags"</label>
                    <input type="text" name="tags" style=INPUT_STYLE
                        placeholder="swap, atomic (comma-separated)"/>

                    <button
                        type="submit"
                        disabled=move || pending.get()
                        style="width: 100%; margin-top: 12px; padding: 8px; \
                               background: #6c63ff; color: #fff; border: none; \
                               border-radius: 4px; cursor: pointer; font-size: 13px; \
                               font-weight: 600;"
                    >
                        {move || if pending.get() { "Creating..." } else { "Create Listing" }}
                    </button>
                </ActionForm>

                {move || {
                    value.get().and_then(|r| r.err()).map(|e| {
                        view! {
                            <p style="color: #f72585; font-size: 12px; margin-top: 8px;">
                                {e.to_string()}
                            </p>
                        }
                    })
                }}
            </div>
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
    let search = RwSignal::new(String::new());
    let version = expect_context::<RwSignal<usize>>();

    let listings = Resource::new(
        move || (search.get(), version.get()),
        |(s, _)| list_listings(s),
    );

    view! {
        <h1 style="color: #6c63ff;">"Listings"</h1>
        <input
            type="text"
            placeholder="Search by address, UTXO, or tag..."
            prop:value=move || search.get()
            on:input=move |ev| search.set(event_target_value(&ev))
            style="width: 100%; max-width: 500px; padding: 10px; background: #1a1a2e; \
                   border: 1px solid #333; color: #e0e0e0; border-radius: 4px; \
                   margin-bottom: 16px; box-sizing: border-box;"
        />
        <Suspense fallback=|| view! { <p>"Loading listings..."</p> }>
            {move || Suspend::new(async move {
                match listings.await {
                    Ok(items) if items.is_empty() => {
                        view! { <p>"No listings found."</p> }.into_any()
                    }
                    Ok(items) => {
                        items
                            .into_iter()
                            .map(|l| view! { <ListingCard listing=l/> })
                            .collect_view()
                            .into_any()
                    }
                    Err(e) => {
                        view! {
                            <p style="color: #f72585;">"Error: " {e.to_string()}</p>
                        }
                        .into_any()
                    }
                }
            })}
        </Suspense>
    }
}

#[component]
fn ListingCard(listing: Listing) -> impl IntoView {
    let Listing {
        id,
        utxo_a,
        amount_a_sats,
        address_a,
        utxo_b,
        amount_b_sats,
        address_b,
        network,
        tags,
        ..
    } = listing;

    let utxo_b_view = utxo_b.map(|u| {
        view! {
            <p style="margin: 4px 0;">
                "UTXO B: " <code>{u}</code>
            </p>
        }
    });
    let amount_b_view = amount_b_sats.map(|a| {
        view! {
            <p style="margin: 4px 0;">
                "Amount B: " {a} " sats"
            </p>
        }
    });
    let tags_view = (!tags.is_empty()).then(|| {
        let chips = tags
            .into_iter()
            .map(|t| {
                view! {
                    <span style="background: #f72585; color: #fff; padding: 2px 8px; \
                                 border-radius: 12px; font-size: 11px;">
                        {t}
                    </span>
                }
            })
            .collect_view();
        view! {
            <div style="margin-top: 8px; display: flex; gap: 4px; flex-wrap: wrap;">
                {chips}
            </div>
        }
    });

    view! {
        <div style="background: #1a1a2e; border-radius: 8px; padding: 16px; \
                     margin-bottom: 12px; max-width: 600px;">
            <div style="display: flex; justify-content: space-between; \
                        align-items: center; margin-bottom: 8px;">
                <span style="color: #6c63ff; font-weight: 700;">
                    "#" {id}
                </span>
                <span style="background: #6c63ff; color: #fff; padding: 2px 8px; \
                             border-radius: 4px; font-size: 12px;">
                    {network}
                </span>
            </div>
            <div style="font-size: 13px; color: #ccc;">
                <p style="margin: 4px 0;">
                    "UTXO A: " <code>{utxo_a}</code>
                </p>
                <p style="margin: 4px 0;">
                    "Amount A: " {amount_a_sats} " sats"
                </p>
                <p style="margin: 4px 0;">
                    "Address A: "
                    <code style="word-break: break-all;">{address_a}</code>
                </p>
                {utxo_b_view}
                {amount_b_view}
                <p style="margin: 4px 0;">
                    "Address B: "
                    <code style="word-break: break-all;">{address_b}</code>
                </p>
            </div>
            {tags_view}
        </div>
    }
}
