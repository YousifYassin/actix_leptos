use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Basic_layout_fn() -> impl IntoView {
    let name: String = "Raiden".to_string();
    let my_title: String = String::from("Basic layout");
    view! {
        <Title text= {my_title}/>
        <h1>"hi " {name}</h1>
        <p>"This is my very first website"</p>
    }
}
