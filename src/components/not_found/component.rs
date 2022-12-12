use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html!{
        <div class="not-found">
            <div class="not-found-text">
                <h1>{"404 Not found"}</h1>
                <h5>{"Not all those who wander are lost - Tolkien"}</h5>
            </div>
        </div>
    }
}