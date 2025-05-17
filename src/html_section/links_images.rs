use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn Links_images_fn() -> impl IntoView {
    let test_head: String = format!("links and images");
    view! {
        <Meta name="discription" content="images, links" />
        <Title text="Images & Title" />
        <h1 inner_html=test_head></h1>

        <!-- "External links" -->
        <h2>"External links"</h2>
        <a href="https://www.google.com">"Click for google"</a>
        <br />
        <a href="https://www.google.com" target="_blank">
            "Click for google in new tab"
        </a>

        <!-- "Internal links" -->
        <h2>"Internal links"</h2>
        <a href="/typography">"Typography"</a>
        <br />

        <!-- "local image" -->
        <img src="/imgs/nature.jpg" alt="nature image" width="300" height="200" />
        <br />
        <!-- "External image" -->
        <img src="https://picsum.photos/200" alt="My image 2" />
    }
}
