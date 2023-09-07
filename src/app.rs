use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // NOTE To cause an error, you can enable `BrowserRouter` here,
    // `tracing` in main, or both.
    html! {
        <main class="container">
            // <BrowserRouter>
                {"Test"}
            // </BrowserRouter>
        </main>
    }
}
