use yew::{function_component, html, Properties, virtual_dom::VNode, use_state_eq, Html};

use crate::helpers::input_macro::button_switch;

#[derive(Properties, PartialEq)]
pub struct AdminModuleProps {
    pub title: String,
    pub module: VNode
}

#[function_component(AdminModule)]
pub fn admin_module(props: &AdminModuleProps) -> Html {
    let show_module = use_state_eq(|| false);
    let callback = button_switch(&show_module);

    html! {
        <div>
            <div>
                <h4>{&props.title}</h4>
                <button onclick={callback}>
                    <i class="bi bi-caret-down"></i>
                </button>
            </div>
            {if *show_module{
                html!{
                    props.module.clone()
                }
            }else{
                html!{}
            }}
        </div>
    }
}