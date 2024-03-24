use leptos::*;

struct Work<T: AsRef<str>, K: AsRef<[T]>> {
    title: T,
    stack: K,
    img: T,
    link: T,
}

const WORKS: &[Work<&str, &[&str]>] = &[
    Work {
        title: "Portfolio",
        stack: &["Rust (WASM)", "HTML", "CSS", "GIT"],
        img: "/static/img/portfolio.png",
        link: "#",
    },
    Work {
        title: "ERP System",
        stack: &[
            "Python (FLASK)",
            "HTML",
            "CSS",
            "GIT",
            "SQL",
            "JAVASCRIPT (VUE3)",
        ],
        img: "/static/img/NDA.jpg",
        link: "#",
    },
];

fn zero_fill_string(count: usize, value: String) -> String {
    "0".repeat(count - value.len()) + &value
}

#[component]
pub fn Works(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="works" class="works-frame">
            <h1>Works</h1>
            <div class="works">
            {
                WORKS.iter().enumerate().map(|(i, work)| view! { cx,
                    <a href={work.link} class="work">
                        <span>
                            <p>{zero_fill_string(WORKS.len().to_string().len() + 1, (i + 1).to_string())}</p>
                            <p>{work.title}</p>
                        </span>
                        <div>
                            <img src={work.img} />
                            <h2>Stack</h2>
                            <ul>
                            {
                                work.stack.iter().map(|item| view! {cx,
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
