use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use serde::{Deserialize, Serialize};

#[component]
pub fn Block_inline() -> impl IntoView {
    let submit = ServerAction::<PersonForm>::new();

    let data_set = (1..=6)
        .map(|num| format!("<h{0}>Heading {0} </h{0}>", num))
        .collect::<Vec<String>>()
        .join("");

    let two_raw = (1..=2)
        .map(|_| format!("<li> item </li>"))
        .collect::<Vec<String>>()
        .join("");

    view! {
        <Meta name="discription" content="Inline Blocks" />
        <Title text="Inline BLOCKS" />
        <div inner_html=data_set></div>
        <p style="border:2px green solid">
            ""
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Cras ultricies ligula sed magna dictum porta."
            ""
        </p>
        <p style="border:2px green solid">"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Cras ultricies ligula sed magna dictum porta."</p>

        <a href="#" style="border:2px red solid">"A link "</a>
        <a href="#" style="border:2px red solid">"A link "</a>

        <ul inner_html=two_raw>
        </ul>

        <ActionForm action=submit>
            <div>
                <label for="first_name">"Frist name"</label><br/>
                <input id="first_name" type="text" name="first_name" />
            </div>
            <div>
                <label for="last_name">"last name"</label><br/>
                <input id="last_name" type="text" name="last_name" />
            </div>
            <input type="submit" />
        </ActionForm>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    first_name: String,
    last_name: String,
}

#[server]
async fn person_form(first_name: String, last_name: String) -> Result<(), ServerFnError> {
    let person = Person {
        first_name,
        last_name,
    };
    println!("{person:#?}");
    Ok(())
}
