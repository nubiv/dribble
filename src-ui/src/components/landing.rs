use leptos::html::{Textarea, Video};
use leptos::{
    component, create_effect, create_node_ref, log,
    use_context, view, IntoView, NodeRef, Scope, SignalGet,
    SignalSet,
};
use web_sys::window;

use crate::app::{InMeetingContext, RtcConnectionContext};
use crate::utils::rtc::{
    answer_offer, create_offer, init_connection,
};

#[component]
pub fn LandingPage(
    cx: Scope,
    local_stream_ref: NodeRef<Video>,
    remote_stream_ref: NodeRef<Video>,
) -> impl IntoView {
    let in_meeting =
        use_context::<InMeetingContext>(cx).unwrap().0;
    let set_in_meeting =
        use_context::<InMeetingContext>(cx).unwrap().1;
    let rtc_pc =
        use_context::<RtcConnectionContext>(cx).unwrap().0;
    let set_rtc_pc =
        use_context::<RtcConnectionContext>(cx).unwrap().1;
    let local_sdp_ref: NodeRef<Textarea> =
        create_node_ref(cx);
    let remote_sdp_ref: NodeRef<Textarea> =
        create_node_ref(cx);
    // let ice_candidate_ref: NodeRef<Textarea> =
    //     create_node_ref(cx);

    let on_answer_offer = move |_| {
        leptos::spawn_local(async move {
            let remote_sdp_input_el =
                remote_sdp_ref.get().unwrap();
            let remote_sdp = remote_sdp_input_el.value();
            // log!("remote_sdp: {:?}", remote_sdp);

            if remote_sdp.is_empty() {
                log!("Remote code is required.");
                return;
            }

            match rtc_pc.get() {
                Some(pc) => {
                    // if let Err(e) = answer_offer(
                    //     &remote_sdp,
                    //     &pc,
                    //     local_stream_ref,
                    //     remote_stream_ref,
                    //     local_sdp_ref,
                    //     rtc_pc,
                    // )
                    // .await
                    // {
                    //     log!("error: {:?}", e);
                    // };
                }
                None => {
                    log!("creating new connection");
                    let pc = init_connection().unwrap();
                    // if let Err(e) = answer_offer(
                    //     &remote_sdp,
                    //     &pc,
                    //     local_stream_ref,
                    //     remote_stream_ref,
                    //     local_sdp_ref,
                    //     rtc_pc,
                    // )
                    // .await
                    // {
                    //     log!("error: {:?}", e);
                    // };
                    set_rtc_pc.set(Some(pc));
                }
            }

            if !in_meeting.get() {
                set_in_meeting.set(true);
            }
        })
    };

    let on_generate_offer = move |_| {
        leptos::spawn_local(async move {
            let pc = init_connection().unwrap();
            // if let Err(e) = create_offer(
            //     &pc,
            //     local_stream_ref,
            //     remote_stream_ref,
            //     local_sdp_ref,
            //     rtc_pc,
            // )
            // .await
            // {
            //     log!("error: {:?}", e);
            // };
            set_rtc_pc.set(Some(pc));

            if !in_meeting.get() {
                set_in_meeting.set(true);
            }
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

    create_effect(cx, move |_| {
        if !in_meeting.get() {
            set_in_meeting.set(false);

            if let Some(el) = local_sdp_ref.get() {
                el.set_value("");
            };
            if let Some(el) = remote_sdp_ref.get() {
                el.set_value("");
            };
        }
    });

    view! { cx,
        <div class="grid grid-cols-2 gap-0 w-full h-[30vh]">
            <div class="col-span-1 p-1">
                <label for="local_sdp">LOCAL: </label>
                <textarea
                    node_ref=local_sdp_ref
                    class="border w-full h-full"
                    type="text"
                    id="local_sdp"
                />
            <button
                class="border mx-2 p-1"
                on:click=on_generate_offer
                >"New offer"</button>
            </div>
            <div class="col-span-1 p-1">
                <label for="remote_sdp">REMOTE: </label>
                <textarea
                    node_ref=remote_sdp_ref
                    class="border w-full h-full"
                    type="text"
                    id="remote_sdp"
                />
                <button
                    class="border mx-2 p-1"
                    on:click=on_answer_offer
                    >"Answer"</button>
            </div>
        </div>
    }
}
