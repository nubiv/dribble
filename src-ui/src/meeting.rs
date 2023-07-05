use leptos::{
    component, html::Video, use_context, view, IntoView,
    NodeRef, Scope, SignalSet,
};

use super::app::InMeetingContext;

#[component]
pub fn Meeting(
    cx: Scope,
    local_stream_ref: NodeRef<Video>,
    remote_stream_ref: NodeRef<Video>,
) -> impl IntoView {
    let set_in_meeting =
        use_context::<InMeetingContext>(cx).unwrap().0;

    view! { cx,
      <div class="grid grid-cols-2 gap-0 w-full">
        <div class="col-span-1">
          <video
            node_ref=remote_stream_ref
            class="w-full p-3"
            autoplay
            controls
          ></video>
        </div>

        <div class="col-span-1">
          <video
            node_ref=local_stream_ref
            class="w-full p-3"
            autoplay
            controls
          ></video>
        </div>
      </div>
      // <button
      //   class="px-10"
      //   on:click= move |_| { set_in_meeting.set(false)}
      // >"Quit Session"</button>
    }
}
