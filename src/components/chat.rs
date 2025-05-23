use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    ToggleEmojiPicker,
    AddEmoji(String),
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
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    show_emoji_picker: bool,
}

const EMOJIS: [&str; 24] = [
    "😀", "😂", "😍", "🤔", "😎", "🚀", "⚡", "🔥", 
    "💯", "👍", "👎", "❤️", "🎉", "🎊", "💫", "⭐",
    "🌟", "✨", "💎", "🎯", "🎮", "🎵", "🎸", "🎭"
];

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
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            show_emoji_picker: false,
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
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    if !input.value().trim().is_empty() {
                        let message = WebSocketMessage {
                            message_type: MsgTypes::Message,
                            data: Some(input.value()),
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
                    }
                };
                false
            }
            Msg::ToggleEmojiPicker => {
                self.show_emoji_picker = !self.show_emoji_picker;
                true
            }
            Msg::AddEmoji(emoji) => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let current_value = input.value();
                    input.set_value(&format!("{}{}", current_value, emoji));
                    input.focus().ok();
                }
                self.show_emoji_picker = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let toggle_emoji = ctx.link().callback(|_| Msg::ToggleEmojiPicker);

        html! {
            <div class="flex w-screen bg-gradient-to-br from-slate-900 via-gray-900 to-black min-h-screen">
                <div class="flex-none w-72 h-screen bg-black bg-opacity-40 backdrop-blur-md border-r border-purple-500 border-opacity-20">
                    <div class="text-xl p-4 text-gray-200 font-semibold border-b border-purple-500 border-opacity-20 bg-gradient-to-r from-purple-600 to-indigo-600 bg-opacity-10">
                        {"Users"}
                    </div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 bg-gradient-to-r from-gray-800 to-gray-700 bg-opacity-30 backdrop-blur-sm rounded-xl p-3 border border-purple-400 border-opacity-10 hover:border-opacity-30 transition-all duration-300">
                                    <div>
                                        <img class="w-12 h-12 rounded-full border border-purple-400 border-opacity-30" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-sm justify-between">
                                            <div class="text-gray-200 font-medium">{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-gray-400">
                                            {"Hi there!"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col relative">
                    <div class="w-full h-16 border-b border-purple-500 border-opacity-20 bg-gradient-to-r from-purple-600 to-indigo-600 bg-opacity-10 backdrop-blur-md">
                        <div class="text-2xl p-4 text-gray-100 font-bold">{"💬 YewChat"}</div>
                    </div>
                    <div class="w-full grow overflow-auto border-b border-purple-500 border-opacity-20 p-4 pb-20">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html!{
                                    <div class="flex items-end w-4/6 bg-gradient-to-r from-gray-800 to-gray-700 bg-opacity-40 backdrop-blur-sm m-4 rounded-tl-2xl rounded-tr-2xl rounded-br-2xl border border-purple-400 border-opacity-10">
                                        <img class="w-8 h-8 rounded-full m-3 border border-purple-400 border-opacity-30" src={user.avatar.clone()} alt="avatar"/>
                                        <div class="p-3">
                                            <div class="text-sm text-gray-300 font-medium">
                                                {m.from.clone()}
                                            </div>
                                            <div class="text-sm text-gray-200 mt-1">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-2 rounded-lg max-w-sm" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    
                    if self.show_emoji_picker {
                        <div class="absolute bottom-20 left-4 bg-black bg-opacity-70 backdrop-blur-xl rounded-2xl p-4 border border-purple-400 border-opacity-20 shadow-2xl shadow-purple-500/10">
                            <div class="grid grid-cols-6 gap-2">
                                {
                                    EMOJIS.iter().map(|&emoji| {
                                        let add_emoji = ctx.link().callback({
                                            let emoji = emoji.to_string();
                                            move |_| Msg::AddEmoji(emoji.clone())
                                        });
                                        html! {
                                            <button 
                                                onclick={add_emoji}
                                                class="w-10 h-10 text-xl hover:bg-purple-600 hover:bg-opacity-30 rounded-lg transition-all duration-200 flex items-center justify-center"
                                            >
                                                {emoji}
                                            </button>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    }
                    
                    <div class="w-full h-16 flex px-4 items-center bg-black bg-opacity-40 backdrop-blur-md border-t border-purple-500 border-opacity-20 absolute bottom-0">
                        <button 
                            onclick={toggle_emoji}
                            class="p-2 bg-gradient-to-r from-purple-600 to-indigo-600 bg-opacity-20 backdrop-blur-sm w-10 h-10 rounded-full flex justify-center items-center text-gray-200 hover:bg-opacity-30 transition-all duration-300 border border-purple-400 border-opacity-20 mr-3"
                        >
                            {"😀"}
                        </button>
                        <input 
                            ref={self.chat_input.clone()} 
                            type="text" 
                            placeholder="Message" 
                            class="block w-full py-3 pl-4 mx-3 bg-gray-800 bg-opacity-40 backdrop-blur-sm rounded-full outline-none focus:ring-1 focus:ring-purple-400 focus:ring-opacity-50 text-gray-200 placeholder-gray-400 border border-purple-400 border-opacity-20" 
                            name="message" 
                            required=true
                        />
                        <button 
                            onclick={submit} 
                            class="p-3 bg-gradient-to-r from-purple-600 to-indigo-600 w-10 h-10 rounded-full flex justify-center items-center text-white hover:from-purple-500 hover:to-indigo-500 transition-all duration-300 shadow-lg shadow-purple-500/20"
                        >
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white w-5 h-5">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}