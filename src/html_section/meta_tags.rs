use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn Meta_tags_fn() -> impl IntoView {
    view! {
        <Title text="meta & tags" />
        <Meta name="description" content="This is just for test my test web site" />

        // This about 8 - 12 keywords we can use
        <Meta name="keywords" content="web development, coding, html, css" />
        // This for search engine to do not follow or index
        <Meta name="rebots" content="NOINDEX, NOFOLLOW" />
        <main>
            <h1>"Hello world"</h1>
        </main>
    }
}
