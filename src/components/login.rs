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
        <div class="bg-gradient-to-br from-slate-900 via-gray-900 to-black flex w-screen h-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <div class="bg-black bg-opacity-30 backdrop-blur-md rounded-3xl p-8 border border-purple-400 border-opacity-20 shadow-2xl shadow-purple-500/10">
                    <h1 class="text-4xl font-bold text-center text-gray-100 mb-8">
                        {"💬 YewChat"}
                    </h1>
                    <form class="m-4 flex">
                        <input 
                            {oninput} 
                            class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-200 border-purple-400 border-opacity-30 bg-gray-800 bg-opacity-40 backdrop-blur-sm placeholder-gray-400 outline-none focus:ring-1 focus:ring-purple-400" 
                            placeholder="Username" 
                        />
                        <Link<Route> to={Route::Chat}> 
                            <button 
                                {onclick} 
                                disabled={username.len()<1} 
                                class="px-8 rounded-r-lg bg-gradient-to-r from-purple-600 to-indigo-600 text-white font-bold p-4 uppercase border-purple-600 border-t border-b border-r hover:from-purple-500 hover:to-indigo-500 transition-all duration-300 disabled:opacity-50" 
                            >
                                {"Go Chatting!"}
                            </button>
                        </Link<Route>>
                    </form>
                </div>
            </div>
        </div>
    }
}