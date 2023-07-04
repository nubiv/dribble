use leptos::html::Video;
use leptos::{
    component, create_node_ref, create_signal,
    provide_context, view, IntoView, NodeRef, Scope,
    SignalGet,
};

use crate::landing_page::LandingPage;
use crate::meeting::Meeting;

#[derive(Clone)]
pub struct InMeetingContext(pub leptos::WriteSignal<bool>);

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (in_meeting, set_in_meeting) =
        create_signal(cx, false);
    provide_context(cx, InMeetingContext(set_in_meeting));

    let local_stream_ref: NodeRef<Video> =
        create_node_ref(cx);
    let remote_stream_ref: NodeRef<Video> =
        create_node_ref(cx);

    view! { cx,
            <main class="mx-[30%] mt-20">
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
