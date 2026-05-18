#![recursion_limit = "512"]

mod components;
mod services;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use components::chat::Chat;
use components::login::Login;
use components::welcome::Welcome;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Welcome,
    #[at("/login")]
    Login,
    #[at("/chat")]
    Chat,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub type User = Rc<UserInner>;

#[derive(Debug, PartialEq)]
pub struct UserInner {
    pub username: RefCell<String>,
}

#[function_component(Main)]
fn main() -> Html {
    let ctx = use_state(|| {
        Rc::new(UserInner {
            username: RefCell::new("initial".into()),
        })
    });

    html! {
        <ContextProvider<User> context={(*ctx).clone()}>
            <BrowserRouter>
                <div class="flex w-screen h-screen">
                    <Switch<Route> render={Switch::render(switch)}/>
                </div>
            </BrowserRouter>
        </ContextProvider<User>>
    }
}

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Welcome => html! {<Welcome />},
        Route::Login => html! {<Login />},
        Route::Chat => html! {<Chat/>},
        Route::NotFound => html! {
            <div class="min-h-screen w-screen bg-slate-950 text-amber-50 flex items-center justify-center px-6">
                <div class="max-w-xl text-center space-y-4">
                    <div class="text-xs uppercase tracking-[0.4em] text-amber-300">{"Signal Lost"}</div>
                    <h1 class="text-5xl font-black">{"404"}</h1>
                    <p class="text-slate-300">
                        {"That page drifted out of the lounge. Head back and rejoin the conversation."}
                    </p>
                    <Link<Route>
                        to={Route::Welcome}
                        classes="inline-flex items-center rounded-full bg-amber-300 px-6 py-3 font-semibold text-slate-950 transition hover:bg-amber-200"
                    >
                        {"Return To Base"}
                    </Link<Route>>
                </div>
            </div>
        },
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}
