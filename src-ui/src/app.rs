use leptos::html::Video;
use leptos::{
    component, create_node_ref, create_signal,
    provide_context, view, IntoView, NodeRef, Scope,
};
use web_sys::RtcPeerConnection;

use crate::components::{LandingPage, Meeting};

#[derive(Clone)]
pub(crate) struct InMeetingContext(
    pub leptos::ReadSignal<bool>,
    pub leptos::WriteSignal<bool>,
);

#[derive(Clone)]
pub struct RtcConnectionContext(
    pub leptos::ReadSignal<Option<RtcPeerConnection>>,
    pub leptos::WriteSignal<Option<RtcPeerConnection>>,
);

#[component]
pub(crate) fn App(cx: Scope) -> impl IntoView {
    let (in_meeting, set_in_meeting) =
        create_signal(cx, false);
    let (rtc_pc, set_rtc_pc) = create_signal::<
        Option<RtcPeerConnection>,
    >(cx, None);
    provide_context(
        cx,
        InMeetingContext(in_meeting, set_in_meeting),
    );
    provide_context(
        cx,
        RtcConnectionContext(rtc_pc, set_rtc_pc),
    );

    let local_stream_ref: NodeRef<Video> =
        create_node_ref(cx);
    let remote_stream_ref: NodeRef<Video> =
        create_node_ref(cx);

    view! { cx,
            <main class="mt-20 w-full h-screen">
            // {
            //     move || if in_meeting.get() {
            //         view! { cx,
            //             <Meeting
            //             local_stream_ref=local_stream_ref
            //             remote_stream_ref=remote_stream_ref
            //             />
            //         }
            //     } else {
            //         view! { cx,
            //             <LandingPage
            //             local_stream_ref=local_stream_ref
            //             remote_stream_ref=remote_stream_ref
            //             />
            //         }
            //     }
            // }
                <Meeting
                local_stream_ref=local_stream_ref
                remote_stream_ref=remote_stream_ref
                />
                <LandingPage
                local_stream_ref=local_stream_ref
                remote_stream_ref=remote_stream_ref
                />
            </main>
    }
}
