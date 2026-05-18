use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");
    let is_ready = username.trim().len() > 1;

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="relative flex min-h-screen w-screen items-center overflow-hidden bg-slate-950 text-amber-50">
            <div class="absolute inset-0 bg-[radial-gradient(circle_at_top_left,_rgba(56,189,248,0.18),_transparent_26%),radial-gradient(circle_at_bottom,_rgba(251,191,36,0.16),_transparent_30%)]"></div>
            <div class="relative mx-auto grid w-full max-w-6xl gap-8 px-6 py-10 lg:grid-cols-[0.95fr_1.05fr] lg:px-10">
                <div class="flex flex-col justify-center space-y-5">
                    <Link<Route> to={Route::Welcome} classes="inline-flex w-fit items-center text-sm font-semibold text-sky-200 transition hover:text-sky-100">
                        {"← Back to landing page"}
                    </Link<Route>>
                    <div class="inline-flex w-fit rounded-full border border-amber-200/20 bg-white/5 px-4 py-2 text-xs uppercase tracking-[0.35em] text-amber-300">
                        {"Step 2: pick a nickname"}
                    </div>
                    <h1 class="max-w-xl text-5xl font-black leading-tight">
                        {"Join the campfire and bring a story."}
                    </h1>
                    <p class="max-w-xl text-lg leading-8 text-slate-300">
                        {"Choose a name, hop into the lounge, and test the app with another browser window to simulate a real conversation."}
                    </p>
                    <div class="grid max-w-lg gap-4 sm:grid-cols-2">
                        <div class="rounded-3xl border border-white/10 bg-white/5 p-4">
                            <div class="text-2xl">{"🎧"}</div>
                            <div class="mt-2 font-bold">{"Lo-fi Energy"}</div>
                            <div class="mt-1 text-sm leading-6 text-slate-300">{"Soft colors and lounge-style copy make the login page feel less like a plain form."}</div>
                        </div>
                        <div class="rounded-3xl border border-white/10 bg-white/5 p-4">
                            <div class="text-2xl">{"🫶"}</div>
                            <div class="mt-2 font-bold">{"Friendly Flow"}</div>
                            <div class="mt-1 text-sm leading-6 text-slate-300">{"The app nudges users through a welcome page, then into chat with a simple ritual."}</div>
                        </div>
                    </div>
                </div>

                <div class="rounded-[2rem] border border-white/10 bg-white/8 p-6 shadow-2xl backdrop-blur lg:p-8">
                    <div class="mb-8 flex items-center justify-between">
                        <div>
                            <div class="text-xs uppercase tracking-[0.35em] text-amber-300">{"Your pass to enter"}</div>
                            <div class="mt-2 text-3xl font-black">{"Lounge Check-In"}</div>
                        </div>
                        <div class="text-5xl">{"☕"}</div>
                    </div>

                    <form class="space-y-5">
                        <label class="block">
                            <span class="mb-3 block text-sm font-semibold text-slate-200">{"Choose your nickname"}</span>
                            <input
                                {oninput}
                                value={(*username).clone()}
                                class="w-full rounded-2xl border border-white/10 bg-slate-900/70 px-5 py-4 text-lg text-white outline-none transition placeholder:text-slate-500 focus:border-amber-300"
                                placeholder="e.g. night-owl, rustacean, study-buddy"
                            />
                        </label>

                        <div class="rounded-2xl border border-amber-200/10 bg-amber-300/10 p-4 text-sm leading-7 text-amber-50/90">
                            {
                                if is_ready {
                                    html! { "Nickname looks good. Open another browser window later to simulate a second user and chat with yourself." }
                                } else {
                                    html! { "Tip: use at least 2 characters so your nickname feels distinct in the room." }
                                }
                            }
                        </div>

                        <Link<Route> to={Route::Chat}>
                            <button
                                {onclick}
                                disabled={!is_ready}
                                class="w-full rounded-2xl bg-amber-300 px-6 py-4 text-lg font-bold text-slate-950 transition hover:bg-amber-200 disabled:cursor-not-allowed disabled:bg-slate-700 disabled:text-slate-400"
                            >
                                {"Enter The Chat Lounge"}
                            </button>
                        </Link<Route>>
                    </form>
                </div>
            </div>
        </div>
    }
}
