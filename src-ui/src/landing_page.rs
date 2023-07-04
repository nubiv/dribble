use base64::engine::general_purpose;
use base64::Engine;
use js_sys::Object;
use leptos::html::Video;
use leptos::{
    component, create_node_ref, create_signal,
    event_target_value, log, use_context, view, IntoView,
    NodeRef, Scope, SignalGet, SignalSet,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;

use crate::meeting::{
    LocalStreamContext, RemoteStreamContext,
};

use super::app::InMeetingContext;
use super::tauri_api::invoke;

#[derive(Serialize, Deserialize)]
struct PassphraseArgs<'a> {
    passphrase: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub sdp: String,
}

#[component]
pub fn LandingPage(
    cx: Scope,
    local_stream_ref: NodeRef<Video>,
    remote_stream_ref: NodeRef<Video>,
) -> impl IntoView {
    let (passphrase, set_passphrase) =
        create_signal(cx, String::from(""));
    let (local_sdp, set_local_sdp) =
        create_signal(cx, String::from(""));
    let (remote_sdp, set_remote_sdp) =
        create_signal(cx, String::from(""));

    let set_in_meeting =
        use_context::<InMeetingContext>(cx).unwrap().0;
    // let local_stream =
    //     use_context::<LocalStreamContext>(cx).unwrap().0;
    // let remote_stream =
    //     use_context::<RemoteStreamContext>(cx).unwrap().0;

    let on_answer_offer = move |_| {
        leptos::spawn_local(async move {
            if passphrase.get().is_empty() {
                log!("passphrase is empty");

                return;
            }

            // let args = to_value(&PassphraseArgs {
            //     passphrase: &passphrase.get(),
            // })
            // .unwrap();
            // log!("args: {:?}", args);
            // let _ = invoke("send_passphrase", args).await;

            use crate::rtc::{
                answer_offer, init_connection,
            };

            let pc = init_connection().unwrap();
            let session = Session {
                sdp: passphrase.get(),
            };
            if let Err(e) = answer_offer(
                session,
                &pc,
                local_stream_ref,
                remote_stream_ref,
            )
            .await
            {
                set_passphrase.set(String::from(""));
                log!("error: {:?}", e);
            };

            set_passphrase.set(String::from(""));
        })
    };

    let on_generate_offer = move |_| {
        leptos::spawn_local(async move {
            use crate::rtc::{
                create_offer, init_connection,
                on_ice_candidate,
            };

            let pc = init_connection().unwrap();
            if let Err(e) = create_offer(
                &pc,
                set_local_sdp,
                local_stream_ref,
                remote_stream_ref,
            )
            .await
            {
                log!("error: {:?}", e);
            };
            on_ice_candidate(&pc).unwrap();
        })
    };

    view! { cx,
        <div>
            <h1>"Enter your passphrase to create or join a meeting:"</h1>
            <input
                class="border"
                type="text"
                placeholder="Enter a passphrase..."
                on:input= move |ev| {
                    let v = event_target_value(&ev);
                    let stripped = v.replace("\\r", "\r").replace("\\n", "\n");

                    // log!("v: {:?}", v);
                    // let encoded = general_purpose::STANDARD_NO_PAD.encode(&v);
                    // log!("encoded: {:?}", encoded);

                    set_passphrase.set(stripped);
                }
                prop:value= move || {
                    passphrase.get()
                }
                />
            <p>"Passphrase: " { move || passphrase.get() }</p>
            <p>"Local sdp:" { move || local_sdp.get() }</p>
            <p>"Remote sdp:" { move || remote_sdp.get() }</p>
            <button
                class="border mx-2 p-1"
                on:click= move |_| { set_in_meeting.set(true)}
                >"Enter"</button>
            <button
                class="border mx-2 p-1"
                on:click=on_answer_offer
                >"Send phrase"</button>
            <button
                class="border mx-2 p-1"
                on:click=on_generate_offer
                >"Generate offer"</button>
        </div>
    }
}
