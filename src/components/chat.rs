use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    username: String,
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            username,
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                ),
                            })
                            .collect();
                        true
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        true
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let value = input.value();
                    if value.trim().is_empty() {
                        return false;
                    }

                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(value),
                        data_array: None,
                    };

                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let connected_count = self.users.len();

        html! {
            <div class="min-h-screen w-screen bg-slate-950 text-amber-50">
                <div class="mx-auto flex min-h-screen w-full max-w-7xl flex-col gap-4 p-4 lg:flex-row lg:p-6">
                    <aside class="flex-none rounded-[2rem] border border-white/10 bg-white/5 p-4 shadow-xl backdrop-blur lg:w-80">
                        <div class="rounded-[1.5rem] bg-[radial-gradient(circle_at_top,_rgba(251,191,36,0.18),_transparent_35%),linear-gradient(135deg,_rgba(15,23,42,0.92),_rgba(30,41,59,0.95))] p-5">
                            <div class="text-xs uppercase tracking-[0.35em] text-amber-300">{"Room Status"}</div>
                            <div class="mt-2 text-3xl font-black">{"Campfire Lounge"}</div>
                            <p class="mt-3 text-sm leading-7 text-slate-300">
                                {"A creative Yew chat space for late-night ideas, memes, and surprisingly productive conversations."}
                            </p>
                            <div class="mt-5 rounded-2xl border border-amber-200/10 bg-slate-900/45 p-4">
                                <div class="text-xs uppercase tracking-[0.25em] text-amber-300">{"You entered as"}</div>
                                <div class="mt-2 text-xl font-bold text-amber-50">{self.username.clone()}</div>
                                <div class="mt-3 text-sm text-slate-300">{format!("{} active listener(s) in the lounge.", connected_count)}</div>
                            </div>
                        </div>

                        <div class="px-1 pt-5">
                            <div class="flex items-center justify-between px-2">
                                <div class="text-lg font-bold">{"Currently online"}</div>
                                <div class="rounded-full bg-emerald-400/15 px-3 py-1 text-xs font-semibold uppercase tracking-[0.2em] text-emerald-300">
                                    {connected_count}
                                </div>
                            </div>
                            {
                                self.users.clone().iter().map(|u| {
                                    html! {
                                        <div class="mt-3 flex items-center gap-3 rounded-2xl border border-white/10 bg-slate-900/60 p-3">
                                            <img class="h-12 w-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                            <div class="flex-grow">
                                                <div class="flex items-center justify-between text-sm">
                                                    <div class="font-semibold">{u.name.clone()}</div>
                                                    <div class="h-2.5 w-2.5 rounded-full bg-emerald-400"></div>
                                                </div>
                                                <div class="mt-1 text-xs text-slate-400">
                                                    {"Ready to share a spark."}
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </aside>

                    <section class="flex min-h-[75vh] grow flex-col rounded-[2rem] border border-white/10 bg-white/5 shadow-xl backdrop-blur">
                        <div class="border-b border-white/10 px-6 py-5">
                            <div class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
                                <div>
                                    <div class="text-xs uppercase tracking-[0.35em] text-sky-300">{"Live conversation"}</div>
                                    <div class="mt-2 text-3xl font-black">{"The Signal Board"}</div>
                                </div>
                                <div class="rounded-2xl border border-sky-200/10 bg-slate-900/50 px-4 py-3 text-sm leading-7 text-slate-300">
                                    {"Share a thought, a study note, or even a GIF link. The room updates in real time."}
                                </div>
                            </div>
                        </div>

                        <div class="grow overflow-auto px-4 py-5 lg:px-6">
                            {
                                if self.messages.is_empty() {
                                    html! {
                                        <div class="flex h-full min-h-[18rem] items-center justify-center">
                                            <div class="max-w-md rounded-[2rem] border border-dashed border-amber-200/20 bg-slate-900/40 p-8 text-center">
                                                <div class="text-5xl">{"🪵"}</div>
                                                <h2 class="mt-4 text-2xl font-bold">{"The fire is warm, but the room is quiet."}</h2>
                                                <p class="mt-3 text-sm leading-7 text-slate-300">
                                                    {"Be the first to post something. A greeting, a thought, or a GIF is enough to get the conversation moving."}
                                                </p>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <>
                                            {
                                                self.messages.iter().map(|m| {
                                                    let avatar = self
                                                        .users
                                                        .iter()
                                                        .find(|u| u.name == m.from)
                                                        .map(|u| u.avatar.clone())
                                                        .unwrap_or_else(|| {
                                                            "https://avatars.dicebear.com/api/adventurer-neutral/guest.svg".into()
                                                        });
                                                    let is_current_user = m.from == self.username;
                                                    let bubble_classes = if is_current_user {
                                                        "ml-auto border border-amber-200/20 bg-amber-300/15"
                                                    } else {
                                                        "mr-auto border border-white/10 bg-slate-900/65"
                                                    };

                                                    html! {
                                                        <div class={classes!("mb-4", "flex", "max-w-3xl", "items-end", "gap-3", "rounded-[1.75rem]", "p-4", bubble_classes)}>
                                                            <img class="h-10 w-10 rounded-full border border-white/10 bg-white" src={avatar} alt="avatar"/>
                                                            <div class="min-w-0">
                                                                <div class="text-sm font-semibold">
                                                                    {m.from.clone()}
                                                                </div>
                                                                <div class="mt-2 break-words text-sm leading-7 text-slate-200">
                                                                    if m.message.ends_with(".gif") {
                                                                        <img class="mt-2 max-h-72 rounded-2xl" src={m.message.clone()}/>
                                                                    } else {
                                                                        {m.message.clone()}
                                                                    }
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </>
                                    }
                                }
                            }
                        </div>

                        <div class="border-t border-white/10 px-4 py-4 lg:px-6">
                            <div class="flex items-center gap-3 rounded-[2rem] border border-white/10 bg-slate-900/55 p-3">
                                <input
                                    ref={self.chat_input.clone()}
                                    type="text"
                                    placeholder="Drop a thought into the lounge..."
                                    class="block w-full bg-transparent px-3 py-3 text-slate-100 outline-none placeholder:text-slate-500"
                                    name="message"
                                    required=true
                                />
                                <button onclick={submit} class="flex h-12 w-12 items-center justify-center rounded-full bg-sky-400 shadow-sm transition hover:bg-sky-300">
                                    <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                        <path d="M0 0h24v24H0z" fill="none"></path>
                                        <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        }
    }
}
