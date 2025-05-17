use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Lists_tables_fn() -> impl IntoView {
    let list_item = (1..=4)
        .map(|num| format!("<li> Item {0} </li>", num))
        .collect::<Vec<String>>()
        .join("");

    view! {
        <Meta name="discription" content="List, tables" />
        <Title text="Lists & tables" />
        // Unorder lists
        <ul>
            <li>"Item 1"</li>
            <li>"Item 2"</li>
            <li>"Item 3"</li>
            <li>"Item 4"</li>
        </ul>
        // <ul inner_html=list_item /* style="list-style: square" */ /* style="list-style: disc" */ style="list-style: circle" /* style="list-style: none" */>
        <ul inner_html=list_item.clone() style="list-style: circle"></ul>

        // Order lists
        // <ol inner_html=list_item.clone() type="1" />
        // <ol inner_html=list_item.clone() type="I" />
        // <ol inner_html=list_item.clone() type="a" />
        <ol inner_html=list_item.clone() type="A" />

        // Nested lists
        <ul>
            <li>"Element 1"</li>
            <li>"Element 2"</li>
            <ol>
                <li>"Item A"</li>
                <li>"Item B"</li>
                <li>"Item C"</li>
            </ol>
            <li>"Element 3"</li>
            <li>"Element 4"</li>
        </ul>

        // Tables
        <table style="width: 600px">
            <thead>
                <tr>
                    <th>"First Name"</th>
                    <th>"Last Name"</th>
                    <th>"Email"</th>
                </tr>
            </thead>

            <tbody>
                <tr>
                    <td>"Raiden"</td>
                    <td>"Dozer"</td>
                    <td>"raidendozer@yahoo.com"</td>
                </tr>
                <tr>
                    <td>"Neo"</td>
                    <td>"Rusty"</td>
                    <td>"neorusty@yahoo.com"</td>
                </tr>
                <tr>
                    <td>"Arch"</td>
                    <td>"Linux"</td>
                    <td>"archlinux@yahoo.com"</td>
                </tr>
            </tbody>
        </table>
    }
}
