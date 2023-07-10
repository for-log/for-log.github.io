use leptos::*;

const WORKS: &[(&str, &[&str], &str, &str)] = &[
    ("Portfolio", &["Rust (WASM)", "HTML", "CSS", "GIT"], "/static/img/portfolio.png", "#"),
];


fn zero_fill_string(count: usize, value: String) -> String {
    vec!["0"; count - value.len()].into_iter().collect::<String>() + &value
}


#[component]
pub fn Works(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="works" class="works-frame">
            <h1>Works</h1>
            <div class="works">
            {
                WORKS.iter().enumerate().map(|(i, (title, stack, img, link))| view! { cx,
                    <a href={*link} class="work">
                        <span>
                            <p>{zero_fill_string(WORKS.len().to_string().len() + 1, (i + 1).to_string())}</p>
                            <p>{*title}</p>
                        </span>
                        <div>
                            <img src={*img} />
                            <h2>Stack</h2>
                            <ul>
                            {
                                stack.iter().map(|item| view! {cx, 
                                    <li>{*item}</li>
                                }).collect_view(cx)
                            }
                            </ul>
                        </div>
                    </a>
                }).collect_view(cx)
            }
            </div>
        </div>
    }
}