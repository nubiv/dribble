use base64::engine::general_purpose;
use base64::Engine;
use leptos::html::{Input, Video, P};
use leptos::{
    component, create_node_ref, log, use_context, view,
    IntoView, NodeRef, Scope,
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
    let set_in_meeting =
        use_context::<InMeetingContext>(cx).unwrap().0;
    let local_sdp_ref: NodeRef<Input> = create_node_ref(cx);
    let remote_sdp_ref: NodeRef<Input> =
        create_node_ref(cx);

    let on_answer_offer = move |_| {
        leptos::spawn_local(async move {
            let remote_sdp_input_el =
                remote_sdp_ref.get().unwrap();
            let remote_sdp = remote_sdp_input_el.value();

            if remote_sdp.is_empty() {
                log!("Remote code is required.");
                return;
            }

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
                log!("error: {:?}", e);
            };
        })
    };

    let on_generate_offer = move |_| {
        leptos::spawn_local(async move {
            let pc = init_connection().unwrap();
            if let Err(e) = create_offer(
                &pc,
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
