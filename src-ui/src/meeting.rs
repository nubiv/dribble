use leptos::{Scope, component, view, IntoView, use_context, SignalSet};

use super::app::InMeetingContext;

#[component]
pub fn Meeting(cx: Scope) -> impl IntoView {

    let set_in_meeting = use_context::<InMeetingContext>(cx).unwrap().0;

    view! { cx,
        <div class="video-container" id="video-container">
        <div class="col"></div>
        <div class="main col">
          <video id="remote-video" autoplay controls></video>
          <div class="controls">
            <button
              type="button"
              id="mute-audio"
              data-status="active"
              class="btn"
            >
              Mute Audio
            </button>
            <button
              type="button"
              id="mute-video"
              data-status="active"
              class="btn"
            >
              Mute Video
            </button>
          </div>
        </div>
  
        // <div class="side-panel col">
        //   <video id="local-video" autoplay controls></video>
        // </div>
        <button on:click= move |_| { set_in_meeting.set(false)}>"Quit"</button>
      </div>
    }
}