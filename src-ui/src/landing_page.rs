use base64::engine::general_purpose;
use base64::Engine;
use leptos::html::{Input, Video, P};
use leptos::{
    component, create_node_ref, create_signal,
    event_target_value, log, use_context, view, IntoView,
    NodeRef, Scope, SignalGet, SignalSet,
};
use serde::{Deserialize, Serialize};
use web_sys::window;

use crate::app::InMeetingContext;
use crate::rtc::{
    answer_offer, create_offer, init_connection,
};

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
    let local_sdp_ref: NodeRef<Input> = create_node_ref(cx);
    let remote_sdp_ref: NodeRef<Input> =
        create_node_ref(cx);

    let on_answer_offer = move |_| {
        leptos::spawn_local(async move {
            // if passphrase.get().is_empty() {
            //     log!("passphrase is empty");

            //     return;
            // }

            // let decoded_utf8 =
            //     general_purpose::STANDARD_NO_PAD
            //         .decode(passphrase.get())
            //         .unwrap();
            // let decoded_str =
            //     String::from_utf8(decoded_utf8).unwrap();
            let remote_sdp_input_el =
                remote_sdp_ref.get().unwrap();
            let remote_sdp = remote_sdp_input_el.value();

            let decoded_utf8 =
                general_purpose::STANDARD_NO_PAD
                    .decode(remote_sdp)
                    .unwrap();
            let decoded_str =
                String::from_utf8(decoded_utf8).unwrap();

            let pc = init_connection().unwrap();
            let session = Session { sdp: decoded_str };
            if let Err(e) = answer_offer(
                session,
                &pc,
                local_stream_ref,
                remote_stream_ref,
                local_sdp_ref,
            )
            .await
            {
                // set_passphrase.set(String::from(""));
                log!("error: {:?}", e);
            };

            // set_passphrase.set(String::from(""));
        })
    };

    let on_generate_offer = move |_| {
        leptos::spawn_local(async move {
            let pc = init_connection().unwrap();
            if let Err(e) = create_offer(
                &pc,
                set_local_sdp,
                local_stream_ref,
                remote_stream_ref,
                local_sdp_ref,
            )
            .await
            {
                log!("error: {:?}", e);
            };
        })
    };

    let on_copy_local_sdp = move || {
        let local_sdp_el = local_sdp_ref.get().unwrap();

        let window = window().unwrap();
        let navigator = window.navigator();
        log!("navigator: {:?}", navigator);

        // need to find other workarounds for this
        #[cfg(web_sys_unstable_apis)]
        {
            let clipboard = navigator.clipboard().unwrap();
            let text = clipboard
                .read_text(&local_sdp_el.inner_html());
            log!("text: {:?}", text);
        }
    };

    view! { cx,
        <div class="px-[30%]">
            // <h1>"Enter your passphrase to create or join a meeting:"</h1>
            // <input
            //     class="border"
            //     type="text"
            //     placeholder="Enter a passphrase..."
            //     on:input= move |ev| {
            //         let v = event_target_value(&ev);
            //         let stripped = v.replace("\\r", "\r").replace("\\n", "\n");

            //         set_passphrase.set(stripped);
            //     }
            //     prop:value= move || {
            //         passphrase.get()
            //     }
            //     />
            <div class="m-2">
                <label for="local_sdp">LOCAL: </label>
                <input
                    node_ref=local_sdp_ref
                    class="border w-auto h-auto"
                    type="text"
                    id="local_sdp"
                />
            </div>
            <div class="m-2">
                <label for="remote_sdp">REMOTE: </label>
                <input
                    node_ref=remote_sdp_ref
                    class="border w-auto h-auto"
                    type="text"
                    id="remote_sdp"
                />
            </div>
            // <p node_ref=local_sdp_ref class="w-1/2 overflow-auto">"Local:" { move || local_sdp.get() }</p>
            // <p class="w-1/2 overflow-auto">"Remote: " { move || passphrase.get() }</p>
            // <button
            //     class="border mx-2 p-1"
            //     on:click=on_copy_local_sdp
            //     >"Copy local sdp"</button>
            // <p class="w-1/2 overflow-auto">"Remote sdp:" { move || remote_sdp.get() }</p>
            // <button
            //     class="border mx-2 p-1"
            //     on:click= move |_| { set_in_meeting.set(true)}
            //     >"Enter"</button>
            <button
                class="border mx-2 p-1"
                on:click=on_generate_offer
                >"New offer"</button>
            <button
                class="border mx-2 p-1"
                on:click=on_answer_offer
                >"Answer"</button>
        </div>
    }
}
