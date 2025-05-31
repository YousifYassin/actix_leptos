use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Id_Class_fn() -> impl IntoView {
    view! {
        <Meta name="discription" content="ID , Classes" />
        <Title text="Div, span, id, class" />
        <style>
            ".card {
                border: 1px solid #ccc;
                background: #f4f4f4;
                padding: 20px;
                margin-bottom: 10px;
            }
             .enhance {
                 color: yellow;
                 background-color: black;
             }
            "
        </style>

        <div id="main-header">
            <h1>"Welcome to my website"</h1>
            <p>"A site About Me"</p>
        </div>
        <ul id="main nav">
            <li><a href="#">"Home"</a></li>
            <li><a href="#">"About"</a></li>
            <li><a href="#">"Contact"</a></li>
        </ul>

        <div id="about" class="card">
            <h3>"About"</h3>
            <p>"""Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut """<span class="enhance">"enim ad minim veniam,"</span>""" quis nostrud exercitation ullamco laboris nisi ut."""</p>
        </div>

        <div id="contact" class="card">
            <h3>"Contact Me"</h3>
            <ul>
                <li>"Address: 50 Main st, Bosten MA"</li>
                <li>"Phone: (123) 456-789"</li>
                <li>"Email: me@somethingcool.com"</li>
            </ul>
        </div>

        <div id="footer">
            <p>"Copyright \u{00A9} 2019"</p>
        </div>
    }
}
