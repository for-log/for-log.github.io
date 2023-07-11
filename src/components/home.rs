use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="home" class="home-frame">
            <h1>
                <span style="background: #F4511E">{"Rust"}</span>
                {" & "}
                <span style="background: #1EC2F4">{"Python"}</span>
            </h1>
            <h1>{"Developer"}</h1>
        </div>
    }
}