use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

use crate::html_section::{
    basic_layout::Basic_layout_fn, block_inline::Block_inline, entities::Entities_fn,
    forms_input::Forms_input_fn, id_class::Id_Class_fn, links_images::Links_images_fn,
    lists_tables::Lists_tables_fn, meta_tags::Meta_tags_fn, typography::Typography_fn,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/actix_leptos.css" />

        // sets the document title
        // <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            // <main>
            <Routes fallback=move || "Not found.">
                <Route path=StaticSegment("") view=HomePage />
                <Route path=StaticSegment("/basic_layout") view=Basic_layout_fn />
                <Route path=StaticSegment("/meta_tags") view=Meta_tags_fn />
                <Route path=StaticSegment("/typography") view=Typography_fn />
                <Route path=StaticSegment("/links_images") view=Links_images_fn />
                <Route path=StaticSegment("/lists_tables") view=Lists_tables_fn />
                <Route path=StaticSegment("/forms_input") view=Forms_input_fn />
                <Route path=StaticSegment("/block_inline") view=Block_inline />
                <Route path=StaticSegment("/id_class") view=Id_Class_fn />
                <Route path=StaticSegment("/entities") view=Entities_fn />
                <Route path=WildcardSegment("any") view=NotFound />
            </Routes>
        // </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <Title text="Welcome to Leptos" />
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
