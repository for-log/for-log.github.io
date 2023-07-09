use yew::{Properties, function_component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
    pub name: String,
    pub images: Vec<String>,
    pub description: String
}

#[function_component]
pub fn Work(props: &Props) -> Html {

    html! {
        <div class="works__item">
            <span style={format!("background-image: url(http://localhost:3000/{});", props.images[0])}>
                <p class="item__title">{props.name.clone()}</p>
            </span>
        </div>
    }
}