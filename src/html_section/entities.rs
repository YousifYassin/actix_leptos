use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Entities_fn() -> impl IntoView {
    view! {
        <Meta name="description" content="entities" />
        <Title text="Entities" />
        <!-- "none breaking space &nbsp;" -->
        <p>"Hello my name is \u{00A0} \u{00A0} \u{00A0} Raiden"</p>
        <!--"Less than &lt;"-->
        <p>" 2 \u{003C} 5"</p>
        <!--"Greater than &gt;"-->
        <p>" 2 \u{003E} 5"</p>
        <!--"copyright &copy;"-->
        <p>" Copyright \u{00A9} "</p>
        <!--"trademark &reg;"-->
        <p>"Trade mark \u{2122}"</p>
        <!--"Cent &cent;"-->
        <p>"Cent \u{00A2}"</p>

        <!--"Computer Code"-->
        <code>
            """println!(" This is Rust code ");"""<br/>
            """\u{003C}?php echo 'hello' \u{003E}"""
        </code>
        <p>"""You can save file by pressing "<kbd>" Ctrl + S "</kbd>" """</p>
    }
}
