use yew::{function_component, html, use_state_eq, virtual_dom::VNode, Html, Properties};

use crate::helpers::input_macro::button_switch;

#[derive(Properties, PartialEq)]
pub struct AdminModuleProps {
    pub title: String,
    pub module: VNode,
}

#[function_component(AdminModule)]
pub fn admin_module(props: &AdminModuleProps) -> Html {
    let show_module = use_state_eq(|| false);
    let callback = button_switch(&show_module);

    html! {
        <div stlye="display: block; padding: 1vh;">
            <div class="admin-header">
                <h4>{&props.title}</h4>
                <button onclick={callback} class="admin-dropdown-button">
                    <i class={format!("bi bi-caret-{}", if *show_module{"up"}else{"down"})}></i>
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
