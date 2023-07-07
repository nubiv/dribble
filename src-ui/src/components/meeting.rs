use leptos::SignalGet;
use leptos::{
    component, html::Video, use_context, view, IntoView,
    NodeRef, Scope, SignalSet,
};
use wasm_bindgen::JsCast;
use web_sys::MediaStreamTrack;

use crate::app::{InMeetingContext, RtcConnectionContext};
use crate::rtc::track_display_stream;

#[component]
pub fn Meeting(
    cx: Scope,
    local_stream_ref: NodeRef<Video>,
    remote_stream_ref: NodeRef<Video>,
) -> impl IntoView {
    let set_in_meeting =
        use_context::<InMeetingContext>(cx).unwrap().1;
    let rtc_pc =
        use_context::<RtcConnectionContext>(cx).unwrap().0;
    let set_rtc_pc =
        use_context::<RtcConnectionContext>(cx).unwrap().1;

    let on_screen_sharing = move |_| {
        // leptos::spawn_local(async move {
        //     track_display_stream().await.unwrap();
        // });
    };

    let on_quit = move |_| {
        if let Some(pc) = rtc_pc.get() {
            pc.close();

            // TODO: somehow need to shut down sound track as well
            let local_stream_el = local_stream_ref
                .get()
                .expect("cant find local stream element");
            close_track(&local_stream_el);

            let remote_stream_el = remote_stream_ref
                .get()
                .expect("cant find local stream element");
            close_track(&remote_stream_el);
        };
        set_rtc_pc.set(None);
        set_in_meeting.set(false);
    };

    fn close_track(el: &leptos::HtmlElement<Video>) {
        if let Some(stream) = el.src_object() {
            let tracks = stream.get_tracks();
            tracks.iter().for_each(|track| {
                track
                    .dyn_into::<MediaStreamTrack>()
                    .unwrap()
                    .stop();
            });
        };
        el.set_src_object(None);
    }

    view! { cx,
      <div class="grid grid-cols-2 gap-0 w-full h-auto">
        <div class="col-span-1">
          <video
            node_ref=local_stream_ref
            class="w-full p-3"
            autoplay
            controls
          >
            <p>"Sorry, your browser doesn't support embedded videos"</p>
          </video>
          <button
            class="border px-5 mx-1"
            on:click=on_quit
            >"Quit Session"</button>
          <button
            class="border px-5 mx-1"
            on:click=on_screen_sharing
            >"Share Screen"</button>
        </div>

        <div class="col-span-1">
          <video
            node_ref=remote_stream_ref
            class="w-full p-3"
            autoplay
            controls
          >
            <p>"Sorry, your browser doesn't support embedded videos"</p>
          </video>
        </div>
      </div>
    }
}
