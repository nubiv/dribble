use leptos::{
    component, event_target_value, use_context, view,
    IntoView, Scope, SignalSet,
};
use web_sys::Event;

use crate::app::{MediaOption, MediaOptionContext};

#[component]
pub(crate) fn ConfigPanel(cx: Scope) -> impl IntoView {
    let set_media_option =
        use_context::<MediaOptionContext>(cx).unwrap().1;

    let on_select = move |e: Event| {
        let value = event_target_value(&e);

        if value == "0" {
            set_media_option.set(MediaOption::FileTransfer);
        } else if value == "1" {
            set_media_option.set(MediaOption::WithVideo);
        } else if value == "2" {
            set_media_option.set(MediaOption::WithAudio);
        }
    };

    view! { cx,
            <div class="flex flex-col h-[30%] mt-4">
                <select class="rounded-lg bg-gray-100 border-blue-400 text-blue-400 m-1 hover:cursor-pointer p-3" on:change=on_select>
                    <option value="0" default>"File Transfer"</option>
                    <option value="1">"With Voice Chat"</option>
                    <option value="2">"With Audio Chat"</option>
                </select>
            </div>
    }
}
