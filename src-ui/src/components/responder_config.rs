use leptos::{
    component, create_node_ref, create_signal,
    html::Textarea, log, use_context, view, IntoView,
    NodeRef, Scope, SignalGet, SignalSet,
};

use crate::{
    app::{
        AppState, AppStateContext, DataChannelContext,
        LocalStreamRef, MediaOption, MediaOptionContext,
        MediaStreamContext, RemoteStreamRef,
        RtcConnectionContext,
    },
    components::ConfigPanel,
    utils::rtc::{
        answer_offer, init_media_stream, setup_datachannel,
        track_ice_candidate_event, track_local_stream,
        track_remote_stream,
    },
};

#[derive(Clone)]
enum ConfigState {
    ConfigPanel,
    LocalSDP,
    RemoteSDP,
}

#[component]
pub(crate) fn ResponderConfig(cx: Scope) -> impl IntoView {
    let set_app_state =
        use_context::<AppStateContext>(cx).unwrap().1;
    let (config_state, set_config_state) =
        create_signal(cx, ConfigState::ConfigPanel);
    let rtc_pc =
        use_context::<RtcConnectionContext>(cx).unwrap().0;
    let set_media_stream =
        use_context::<MediaStreamContext>(cx).unwrap().1;
    let set_dc =
        use_context::<DataChannelContext>(cx).unwrap().1;
    let local_sdp_ref: NodeRef<Textarea> =
        create_node_ref(cx);
    let remote_sdp_ref: NodeRef<Textarea> =
        create_node_ref(cx);
    let local_stream_ref =
        use_context::<LocalStreamRef>(cx).unwrap().0;
    let remote_stream_ref =
        use_context::<RemoteStreamRef>(cx).unwrap().0;
    let media_option =
        use_context::<MediaOptionContext>(cx).unwrap().0;

    let on_remote_sdp_state = move |_| match rtc_pc.get() {
        Some(pc) => {
            leptos::spawn_local(async move {
                track_ice_candidate_event(
                    &pc,
                    local_sdp_ref,
                )
                .expect(
                    "failed to track ice candidate event",
                );

                match media_option.get() {
                    MediaOption::FileTransfer => {
                        log!("default mode");
                    }
                    MediaOption::WithVideo => {
                        log!("with video");
                        let media_stream = init_media_stream(
                                set_media_stream,
                                media_option,
                            )
                            .await
                            .expect("failed to init media stream");
                        track_local_stream(
                            &pc,
                            local_stream_ref,
                            media_stream,
                        )
                        .await
                        .expect(
                            "failed to track local stream",
                        );
                        track_remote_stream(
                            &pc,
                            remote_stream_ref,
                        )
                        .expect(
                            "failed to track remote stream",
                        );
                    }
                    MediaOption::WithAudio => {
                        log!("with audio");
                        let media_stream = init_media_stream(
                                set_media_stream,
                                media_option,
                            )
                            .await
                            .expect("failed to init media stream");
                        track_local_stream(
                            &pc,
                            local_stream_ref,
                            media_stream,
                        )
                        .await
                        .expect(
                            "failed to track local stream",
                        );
                        track_remote_stream(
                            &pc,
                            remote_stream_ref,
                        )
                        .expect(
                            "failed to track remote stream",
                        );
                    }
                };

                setup_datachannel(&pc, set_dc)
                    .expect("failed to setup datachannel");

                set_config_state
                    .set(ConfigState::RemoteSDP);
            });
        }
        None => {
            log!("error: no connection established.");
            set_app_state.set(AppState::Stable);
        }
    };

    let on_local_sdp_state = move |_| {
        match rtc_pc.get() {
            Some(pc) => {
                leptos::spawn_local(async move {
                    let remote_sdp_el =
                        remote_sdp_ref.get().unwrap();
                    let remote_sdp = remote_sdp_el.value();
                    // log!("remote_sdp: {:?}", remote_sdp);

                    if remote_sdp.is_empty() {
                        log!("Remote code is required.");
                        return;
                    }

                    if let Err(e) = answer_offer(
                        &remote_sdp,
                        &pc,
                        local_sdp_ref,
                    )
                    .await
                    {
                        log!("error: {:?}", e);
                    };

                    set_config_state
                        .set(ConfigState::LocalSDP);
                });
            }
            None => {
                log!("error: no connection established.");
                set_app_state.set(AppState::Stable);
            }
        }
    };

    let on_connect = move |_| {
        if let Some(el) = local_sdp_ref.get() {
            el.set_value("");
        };
        if let Some(el) = remote_sdp_ref.get() {
            el.set_value("");
        };

        set_config_state.set(ConfigState::ConfigPanel);
        set_app_state.set(AppState::Connected);
    };

    view! { cx,
            <div
                class="flex flex-col items-center h-full w-full"
                style:display=move || match config_state.get() {
                    ConfigState::ConfigPanel => "flex",
                    _ => "none",
                }
            >
                <ConfigPanel />

                <button
                    class="bg-blue-400 text-white rounded-lg py-2 px-8 mt-8 hover:bg-gray-600"
                    on:click=on_remote_sdp_state
                    >
                    "Next"
                </button>
            </div>

            <div
                class="flex flex-col items-center h-[30%] w-full"
                style:display=move || match config_state.get() {
                    ConfigState::RemoteSDP => "flex",
                    _ => "none",
                }
            >
                <div class="flex flex-col m-auto">
                    <label class="text-blue-700" for="local_sdp">"Remote Key: "</label>
                    <textarea
                        node_ref=remote_sdp_ref
                        class="border-blue-400 border-2 w-[40vw] h-[20vh] rounded-lg p-2"
                        type="text"
                        id="remote_sdp"
                    />
                </div>

                <button
                    class="bg-blue-400 text-white rounded-lg py-2 px-4 mt-8 hover:bg-gray-600"
                    on:click=on_local_sdp_state
                    >
                    "Generate key"
                </button>
            </div>


            <div
                class="flex flex-col items-center h-[30%] w-full"
                style:display=move || match config_state.get() {
                    ConfigState::LocalSDP => "flex",
                    _ => "none",
                }
            >
                <div class="flex flex-col m-auto">
                    <label class="text-blue-700" for="local_sdp">"Local Key: "</label>
                    <textarea
                        node_ref=local_sdp_ref
                        class="border-blue-400 border-2 w-[40vw] h-[20vh] rounded-lg p-2"
                        type="text"
                        id="local_sdp"
                    />
                </div>

                <button
                    class="bg-blue-400 text-white rounded-lg py-2 px-4 mt-8 hover:bg-gray-600"
                    on:click=on_connect
                    >
                    "Connect"
                </button>
            </div>
    }
}
