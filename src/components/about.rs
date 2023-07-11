use leptos::*;

const ABOUT_ITEMS: &[(&str, &[&str], &str)] = &[
    ("Skills", &[
        "Rust", "Python", "HTML", "CSS", 
        "JavaScript", "C/C++", "Git", "Sql"
    ], "/static/img/img4.png"),
    ("Education", &["Perm State University, Applied Mathematics & Informatics [2022-2026]"], "/static/img/img1.png"),
    ("Languages", &["Russian (C2)", "English (B1-B2)"], "/static/img/img2.png"),
    ("Contacts", &["Email: shagabutdinov.david@yandex.ru", "Telegram: @ForderXS"], "/static/img/img3.png"),
];

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="about" class="about-frame">
            <h1>{"About"}</h1>
            {
                ABOUT_ITEMS.iter().map(|(title, items, img)| view! { cx,
                    <div class="item">
                        <img src={*img} />
                        <div class="info">
                            <h3>{*title}</h3>
                            <ul>
                                {
                                    items.iter().map(|x| view! { cx,
                                        <li>{*x}</li>
                                    }).collect_view(cx)
                                }
                            </ul>
                        </div>
                    </div>
                }).collect_view(cx)
            }
        </div>
    }
}