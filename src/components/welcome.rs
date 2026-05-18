use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <main class="min-h-screen w-screen overflow-hidden bg-slate-950 text-amber-50">
            <div class="absolute inset-0 bg-[radial-gradient(circle_at_top,_rgba(251,191,36,0.18),_transparent_32%),radial-gradient(circle_at_bottom_right,_rgba(14,165,233,0.16),_transparent_28%)]"></div>
            <div class="relative mx-auto flex min-h-screen max-w-6xl flex-col justify-between px-6 py-8 lg:px-10">
                <header class="flex items-center justify-between">
                    <div>
                        <div class="text-xs uppercase tracking-[0.4em] text-amber-300">{"YewChat"}</div>
                        <div class="text-sm text-slate-300">{"Campfire edition for real-time conversations"}</div>
                    </div>
                    <Link<Route>
                        to={Route::Login}
                        classes="rounded-full border border-amber-200/40 px-5 py-2 text-sm font-semibold text-amber-100 transition hover:border-amber-200 hover:bg-amber-50 hover:text-slate-950"
                    >
                        {"Enter Lounge"}
                    </Link<Route>>
                </header>

                <section class="grid gap-10 py-12 lg:grid-cols-[1.15fr_0.85fr] lg:items-center">
                    <div class="space-y-6">
                        <div class="inline-flex items-center rounded-full border border-amber-200/20 bg-white/5 px-4 py-2 text-xs uppercase tracking-[0.35em] text-amber-200">
                            {"Creative Mode On"}
                        </div>
                        <h1 class="max-w-3xl text-5xl font-black leading-tight text-balance md:text-6xl">
                            {"A cozy real-time lounge built with Rust, Yew, and a little personality."}
                        </h1>
                        <p class="max-w-2xl text-lg leading-8 text-slate-300">
                            {"This version reimagines the tutorial chat app as a warm digital hangout space. Instead of a plain login form, visitors arrive at a small story-driven landing page before joining the conversation."}
                        </p>
                        <div class="flex flex-wrap gap-4">
                            <Link<Route>
                                to={Route::Login}
                                classes="rounded-full bg-amber-300 px-7 py-4 font-semibold text-slate-950 transition hover:bg-amber-200"
                            >
                                {"Start Chatting"}
                            </Link<Route>>
                            <a
                                href="https://yew.rs"
                                target="_blank"
                                rel="noreferrer"
                                class="rounded-full border border-sky-300/30 px-7 py-4 font-semibold text-sky-100 transition hover:bg-sky-300/10"
                            >
                                {"Explore Yew"}
                            </a>
                        </div>
                    </div>

                    <div class="relative">
                        <div class="absolute -left-6 top-10 h-24 w-24 rounded-full bg-amber-300/20 blur-2xl"></div>
                        <div class="absolute bottom-0 right-0 h-32 w-32 rounded-full bg-sky-400/20 blur-3xl"></div>
                        <div class="relative rounded-[2rem] border border-white/10 bg-white/8 p-6 shadow-2xl backdrop-blur">
                            <div class="mb-4 flex items-center justify-between">
                                <div>
                                    <div class="text-sm uppercase tracking-[0.35em] text-amber-300">{"Tonight's vibe"}</div>
                                    <div class="mt-1 text-2xl font-bold">{"Late-night builder lounge"}</div>
                                </div>
                                <div class="text-4xl">{"🔥"}</div>
                            </div>
                            <div class="space-y-3 text-sm text-slate-200">
                                <div class="rounded-2xl bg-slate-900/70 p-4">
                                    {"\"Ship small. Share ideas. Leave the room brighter than you found it.\""}
                                </div>
                                <div class="grid gap-3 sm:grid-cols-2">
                                    <div class="rounded-2xl border border-amber-200/10 bg-slate-900/50 p-4">
                                        <div class="text-xs uppercase tracking-[0.25em] text-amber-300">{"Mood"}</div>
                                        <div class="mt-2 text-lg font-semibold">{"Curious + Playful"}</div>
                                    </div>
                                    <div class="rounded-2xl border border-sky-200/10 bg-slate-900/50 p-4">
                                        <div class="text-xs uppercase tracking-[0.25em] text-sky-300">{"Built With"}</div>
                                        <div class="mt-2 text-lg font-semibold">{"Yew + WebSockets"}</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="grid gap-4 pb-8 md:grid-cols-3">
                    <div class="rounded-3xl border border-white/10 bg-white/5 p-5">
                        <div class="text-2xl">{"🌙"}</div>
                        <h2 class="mt-3 text-xl font-bold">{"Warm Welcome"}</h2>
                        <p class="mt-2 text-sm leading-7 text-slate-300">
                            {"An extra landing page gives the project a more intentional flow before the user enters the chat."}
                        </p>
                    </div>
                    <div class="rounded-3xl border border-white/10 bg-white/5 p-5">
                        <div class="text-2xl">{"💬"}</div>
                        <h2 class="mt-3 text-xl font-bold">{"Personality First"}</h2>
                        <p class="mt-2 text-sm leading-7 text-slate-300">
                            {"Friendly copy, visual accents, and a stronger theme make the tutorial feel more like a product."}
                        </p>
                    </div>
                    <div class="rounded-3xl border border-white/10 bg-white/5 p-5">
                        <div class="text-2xl">{"⚡"}</div>
                        <h2 class="mt-3 text-xl font-bold">{"Still Real-Time"}</h2>
                        <p class="mt-2 text-sm leading-7 text-slate-300">
                            {"The WebSocket workflow stays the same, so the creative layer doesn’t break the original chat functionality."}
                        </p>
                    </div>
                </section>
            </div>
        </main>
    }
}
