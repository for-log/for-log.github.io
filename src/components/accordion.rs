use web_sys::HtmlElement;
use yew::{function_component, html, Html, Properties, use_state, Callback, use_node_ref};
use crate::backend_answers::AccordionValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub values: Vec<AccordionValue>
}

#[function_component]
pub fn Accordion(props: &Props) -> Html {
    let is_collapsed = use_state(|| false);
    let enumerations_ref = use_node_ref();

    let on_item_click = Callback::from({
        let is_collapsed = is_collapsed.clone();
        let enumerations_ref = enumerations_ref.clone();
        move |_| {
            is_collapsed.set(!*is_collapsed);
            enumerations_ref.cast::<HtmlElement>().map(|element| {
                let height = if *is_collapsed { 0 } else { element.scroll_height() };
                let _ = element.set_attribute("style", &format!("height: {height}px"));
                Some(())
            });
        }
    });

    html! {
        <div class="accordion__item" onclick={on_item_click}>
            <span class="item__control">
                <img src="/static/img/arrow.svg" class="control__rotate" style={format!("transform: rotate({}deg)", 90 * (*is_collapsed as i32))} />
                <h2>{props.name.clone()}</h2>
            </span>
            <div ref={enumerations_ref} class="enumerations" style="height: 0px">
                {
                    props.values.iter().map(|x| {
                        html! {
                            <div class="enumeration-place">
                                <i class={x.icon.clone()}></i>
                                <a>{x.title.clone()}</a>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}