use std::rc::Rc;

use leptos::html::Video;
use leptos::{
    component, create_node_ref, create_signal,
    provide_context, view, IntoView, NodeRef, Scope,
    SignalGet,
};
use web_sys::{
    MediaStream, RtcDataChannel, RtcPeerConnection,
};

use crate::pages::{
    ConfigPage, LandingPage, LoadingPage, LobbyPage,
};

#[derive(Clone)]
pub(crate) struct InMeetingContext(
    pub leptos::ReadSignal<bool>,
    pub leptos::WriteSignal<bool>,
);

#[derive(Clone)]
pub(crate) struct RtcConnectionContext(
    pub leptos::ReadSignal<Option<RtcPeerConnection>>,
    pub leptos::WriteSignal<Option<RtcPeerConnection>>,
);

#[derive(Clone)]
pub(crate) struct MediaStreamContext(
    pub leptos::ReadSignal<Option<Rc<MediaStream>>>,
    pub leptos::WriteSignal<Option<Rc<MediaStream>>>,
);

#[derive(Clone)]
pub(crate) struct DataChannelContext(
    pub leptos::ReadSignal<Option<RtcDataChannel>>,
    pub leptos::WriteSignal<Option<RtcDataChannel>>,
);

#[derive(Clone)]
pub(crate) struct AppStateContext(
    pub leptos::ReadSignal<AppState>,
    pub leptos::WriteSignal<AppState>,
);

#[derive(Clone)]
pub(crate) struct MediaOptionContext(
    pub leptos::ReadSignal<MediaOption>,
    pub leptos::WriteSignal<MediaOption>,
);

#[derive(Clone)]
pub(crate) struct RoleContext(
    pub leptos::ReadSignal<Role>,
    pub leptos::WriteSignal<Role>,
);

#[derive(Clone)]
pub(crate) struct LocalStreamRef(pub NodeRef<Video>);

#[derive(Clone)]
pub(crate) struct RemoteStreamRef(pub NodeRef<Video>);

#[derive(Clone)]
pub(crate) enum AppState {
    Stable,
    Connecting,
    Connected,
    Loading,
}

#[derive(Clone)]
pub(crate) enum Role {
    Initiator,
    Responder,
}

#[derive(Clone, Default)]
pub(crate) enum MediaOption {
    #[default]
    FileTransfer,
    WithVideo,
    WithAudio,
}

#[component]
pub(crate) fn App(cx: Scope) -> impl IntoView {
    let (app_state, set_app_state) =
        create_signal(cx, AppState::Stable);
    let (role, set_role) =
        create_signal(cx, Role::Initiator);
    let (in_meeting, set_in_meeting) =
        create_signal(cx, false);
    let (rtc_pc, set_rtc_pc) = create_signal::<
        Option<RtcPeerConnection>,
    >(cx, None);
    let (media_stream, set_media_stream) =
        create_signal::<Option<Rc<MediaStream>>>(cx, None);
    let (media_option, set_media_option) =
        create_signal(cx, MediaOption::default());
    let (dc, set_dc) =
        create_signal::<Option<RtcDataChannel>>(cx, None);
    provide_context(
        cx,
        AppStateContext(app_state, set_app_state),
    );
    provide_context(
        cx,
        MediaOptionContext(media_option, set_media_option),
    );
    provide_context(cx, RoleContext(role, set_role));
    provide_context(
        cx,
        InMeetingContext(in_meeting, set_in_meeting),
    );
    provide_context(
        cx,
        RtcConnectionContext(rtc_pc, set_rtc_pc),
    );
    provide_context(
        cx,
        MediaStreamContext(media_stream, set_media_stream),
    );
    provide_context(cx, DataChannelContext(dc, set_dc));

    let local_stream_ref: NodeRef<Video> =
        create_node_ref(cx);
    let remote_stream_ref: NodeRef<Video> =
        create_node_ref(cx);
    provide_context(cx, LocalStreamRef(local_stream_ref));
    provide_context(cx, RemoteStreamRef(remote_stream_ref));

    view! { cx,
            <main class="flex flex-col items-center w-screen h-screen p-2">

            <div
                class="grid grid-cols-2 gap-0 w-full h-auto mt-auto"
                style:display=move || match app_state.get() {
                    AppState::Connected => {
                        match media_option.get() {
                            MediaOption::WithVideo => "grid",
                            _ => "none"
                        }
                    },
                    _ => "none",
                }
            >
                <div class="col-span-1">
                    <video
                        node_ref=local_stream_ref
                        class="w-full px-3"
                        autoplay
                        controls
                    >
                    <p>"Sorry, your browser doesn't support embedded videos"</p>
                    </video>
                </div>

                <div class="col-span-1">
                    <video
                        node_ref=remote_stream_ref
                        class="w-full px-3"
                        autoplay
                        controls
                    >
                        <p>"Sorry, your browser doesn't support embedded videos"</p>
                    </video>
                </div>
            </div>

            {
                move || match app_state.get() {
                    AppState::Stable => {
                        view! { cx,
                            <LandingPage />
                            // <LobbyPage />
                        }
                    }
                    AppState::Connecting => {
                        view! { cx,
                            <ConfigPage />
                        }
                    }
                    AppState::Connected => {
                        view! { cx,
                            <LobbyPage />
                        }
                    }
                    AppState::Loading => {
                        view! { cx,
                            <LoadingPage />
                        }
                    }
                }
            }

            </main>
    }
}
