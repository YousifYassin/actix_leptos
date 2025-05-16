use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn Typography_fn() -> impl IntoView {
    let tag_name: String = String::from("Headings, Paragraph, Typography");
    let data_test = (1..=6)
        .map(|num| format!("<h{0}> Heading{0} </h{0}>", num))
        .collect::<Vec<String>>()
        .join("");

    view! {
        <Meta name="description" content="Typography, Paragraph, Heading" />
        <Title text=tag_name />
        // just use h1 one time for page for search engine
        // Heading
        <h1>"Heading 1"</h1>
        <h2>"Heading 2"</h2>
        <h3>"Heading 3"</h3>
        <h4>"Heading 4"</h4>
        <h5>"Heading 5"</h5>
        <h6>"Heading 6"</h6>

        // Paragraph
        <p>"We reach this points"</p>
        <p lang="ar" dir="rtl">
            "بسم الله الرحمن الرحيم"
        </p>
        <!--"Testing comment"-->
        <p>"""Lorem ipsum dolor sit amet, "<strong>"consectetur adipiscing elit."</strong>" Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. "<em>"Ut enim ad minim veniam"</em>", quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."/*Horizental line*/""<hr/>" Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui"<del>" officia deserunt"</del>" mollit anim id est laborum."/* This is line breaker */" "<br/>"Cras ultricies ligula sed magna dictum porta."""</p>
        <hr/>
        <div inner_html=data_test></div>
    }
}
