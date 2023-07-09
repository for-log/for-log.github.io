pub mod components;
pub mod backend_answers;

use gloo_net::http::Request;
use yew::prelude::*;
use crate::{components::{Accordion, Work}, backend_answers::{AccordionElement, ProjectElement}};

const MAIN_IMAGE: &str = "/static/img/avatar.webm";
const DEVELOPER_NAME: &str = "Shagabutdinov David";

#[function_component]
fn App() -> Html {
    let accordions_value = use_state(Vec::new);
    let projects_value = use_state(Vec::new);

    let accordions_value_cloned = accordions_value.clone();
    let projects_value_cloned = projects_value.clone();
    
    use_effect_with_deps(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let accordion_request = Request::get("http://localhost:3000/accordion")
                .send().await;
            
            let response = match accordion_request {
                Ok(resp) => {
                    resp.json::<Vec<AccordionElement>>().await.unwrap()
                },
                Err(e) => panic!("{:?}", e)
            };

            accordions_value_cloned.set(response);

            
            let projects_request = Request::get("http://localhost:3000/projects")
                .query([("offset", "0")])
                .send().await;

            let response = match projects_request {
                Ok(resp) => resp.json::<Vec<ProjectElement>>().await.unwrap(),
                Err(e) => panic!("{:?}", e)
            };

            projects_value_cloned.set(response);
        })
    }, ());

    html! {
        <>
        <div class="me-block">
            <div class="me-block__avatar">
                <img src={MAIN_IMAGE} alt="Forder" />
                <h2>{DEVELOPER_NAME}</h2>
                <h3>{"Developer"}</h3>
            </div>
            <div class="me-block__content">
                <h2>{"Greetings"}</h2>
                <span>
                    <p>{"I am a "}<span class="primary-text">{"programmer"}</span>{". My main stack is "}
                    <span class="primary-text">{"Python"}</span>{", "}<span class="primary-text">{"Rust"}</span>{". 
                    I write different projects for my own use, you can see them on the same site. I know "}
                    <span class="primary-text">{"JavaScript"}</span>{", "}<span class="primary-text">{"Html"}</span>{", "}
                    <span class="primary-text">{"Css"}</span>{", "}<span class="primary-text">{r"C\C++"}</span>{"."}
                    {"This site is written in "}<span class="primary-text">{"Rust (Yew + Axum)"}</span>{". "}
                    {"Besides websites I write bots, parsers and desktop programs."}</p>
                    <p>{"I study at the "}<span class="primary-text">{"Perm State University"}</span>{", in the field of "}<span class="primary-text">{"Applied Mathematics and Informatics"}</span>{"."}
                    {"I position myself as a "}<span class="primary-text">{"backend"}</span>{", but I am trying to develop myself in the "}<span class="primary-text">{"frontend"}</span>{" as well."}</p>
                </span>
            </div>
        </div>
        <div class="info-block">
            <div class="info-block__accordions">
                {accordions_value.iter().map(|x| 
                    html! { <Accordion name={x.name.clone()} values={x.values.clone()} /> 
                }).collect::<Html>()}
            </div>

            <div class="info-block__works">
                <h2>{"Works"}</h2>
                <div class="works__items">
                    {projects_value.iter().map(|x| 
                        html! { <Work id={x.id} images={x.images.clone()} name={x.name.clone()} description={x.description.clone()} /> 
                    }).collect::<Html>()}
                </div>
            </div>
        </div>
        // <div class="game-block">
        //     <Game />
        // </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}