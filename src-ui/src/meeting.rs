use leptos::{
    component, create_node_ref, create_signal, html::Video,
    provide_context, use_context, view, IntoView, NodeRef,
    Scope, SignalSet,
};

use super::app::InMeetingContext;

#[derive(Clone)]
pub struct LocalStreamContext(pub leptos::NodeRef<Video>);

#[derive(Clone)]
pub struct RemoteStreamContext(pub leptos::NodeRef<Video>);

#[component]
pub fn Meeting(
    cx: Scope,
    local_stream_ref: NodeRef<Video>,
    remote_stream_ref: NodeRef<Video>,
) -> impl IntoView {
    let set_in_meeting =
        use_context::<InMeetingContext>(cx).unwrap().0;
    let local_stream: NodeRef<Video> = create_node_ref(cx);
    let remote_stream: NodeRef<Video> = create_node_ref(cx);

    provide_context(cx, LocalStreamContext(local_stream));
    provide_context(cx, RemoteStreamContext(remote_stream));

    view! { cx,
        <div class="">
        <div class="">
          <video node_ref=remote_stream_ref id="remote_stream" autoplay controls></video>
          <div class="">
            <button
              type="button"
              id="mute-audio"
              data-status="active"
              class="btn"
            >
              "Mute Audio"
            </button>
            <button
              type="button"
              data-status="active"
              class="btn"
            >
              "Mute Video"
            </button>
          </div>
        </div>

        <div class="">
          <video node_ref=local_stream_ref id="local_stream" autoplay controls></video>
        </div>
        <button on:click= move |_| { set_in_meeting.set(false)}>"Quit"</button>
      </div>
    }
}
