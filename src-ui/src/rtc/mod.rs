use base64::engine::general_purpose;
use base64::Engine;
use js_sys::{Array, Object, Reflect};
use leptos::{
    html::{Textarea, Video},
    log, NodeRef, SignalGet,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, MediaStream, MediaStreamConstraints,
    RtcConfiguration, RtcIceCandidateInit,
    RtcIceConnectionState, RtcPeerConnection,
    RtcPeerConnectionIceEvent, RtcSdpType,
    RtcSessionDescriptionInit, RtcSignalingState,
    RtcTrackEvent,
};

pub(crate) fn init_connection(
) -> Result<RtcPeerConnection, JsValue> {
    RtcPeerConnection::new_with_configuration(&{
        let ice_servers = Array::new();
        let server_entry = Object::new();
        Reflect::set(
            &server_entry,
            &"urls".into(),
            &"stun:stun.l.google.com:19302".into(),
        )?;
        ice_servers.push(&server_entry);

        let mut rtc_configuration = RtcConfiguration::new();
        rtc_configuration.ice_servers(&ice_servers);
        Reflect::set(
            &rtc_configuration,
            &"iceCandidatePoolSize".into(),
            &3.into(),
        )?;
        rtc_configuration
    })
}

pub(crate) async fn create_offer(
    pc: &RtcPeerConnection,
    local_stream_ref: NodeRef<Video>,
    remote_stream_ref: NodeRef<Video>,
    local_sdp_ref: NodeRef<Textarea>,
    rtc_pc: leptos::ReadSignal<Option<RtcPeerConnection>>,
) -> Result<(), JsValue> {
    track_ice_candidate_event(pc, rtc_pc, local_sdp_ref)?;
    track_local_stream(pc, local_stream_ref).await?;
    track_remote_stream(pc, remote_stream_ref)?;

    let local_offer =
        JsFuture::from(pc.create_offer()).await?;
    let local_offer =
        RtcSessionDescriptionInit::from(local_offer);

    let _ = JsFuture::from(
        pc.set_local_description(&local_offer),
    )
    .await?;
    // log!("local description: {:?}", &local_offer);
    log!("signaling state: {:?}", pc.signaling_state());

    let sdp_js = Reflect::get(&local_offer, &"sdp".into())?;
    let sdp_str = sdp_js.as_string().unwrap();
    let encoded =
        general_purpose::STANDARD_NO_PAD.encode(sdp_str);
    let local_sdp_input_el = local_sdp_ref.get().unwrap();
    local_sdp_input_el.set_value(&encoded);

    Ok(())
}

pub(crate) async fn answer_offer(
    remote_sdp: &str,
    pc: &RtcPeerConnection,
    local_stream_ref: NodeRef<Video>,
    remote_stream_ref: NodeRef<Video>,
    local_sdp_ref: NodeRef<Textarea>,
    rtc_pc: leptos::ReadSignal<Option<RtcPeerConnection>>,
) -> Result<(), JsValue> {
    let splitted: Vec<&str> =
        remote_sdp.split('+').collect();

    let remote_sdp_encoded = splitted[0];
    let remote_sdp_utf8 = general_purpose::STANDARD_NO_PAD
        .decode(remote_sdp_encoded)
        .unwrap();
    let remote_sdp =
        &String::from_utf8(remote_sdp_utf8).unwrap();

    match pc.signaling_state() {
        RtcSignalingState::HaveLocalOffer => {
            let mut remote_offer =
                RtcSessionDescriptionInit::new(
                    RtcSdpType::Answer,
                );
            remote_offer.sdp(remote_sdp);

            let _ = JsFuture::from(
                pc.set_remote_description(&remote_offer),
            )
            .await?;
            // log!("remote description: {:?}", &remote_offer);
            log!(
                "signaling state: {:?}",
                pc.signaling_state()
            );
        }
        RtcSignalingState::Stable => {
            track_ice_candidate_event(
                pc,
                rtc_pc,
                local_sdp_ref,
            )?;
            track_local_stream(pc, local_stream_ref)
                .await?;
            track_remote_stream(pc, remote_stream_ref)?;

            let mut remote_offer =
                RtcSessionDescriptionInit::new(
                    RtcSdpType::Offer,
                );
            remote_offer.sdp(remote_sdp);

            let _ = JsFuture::from(
                pc.set_remote_description(&remote_offer),
            )
            .await?;
            // log!("remote description: {:?}", &remote_offer);
            log!(
                "signaling state: {:?}",
                pc.signaling_state()
            );

            let answer =
                JsFuture::from(pc.create_answer()).await?;
            let answer =
                RtcSessionDescriptionInit::from(answer);
            let _ = JsFuture::from(
                pc.set_local_description(&answer),
            )
            .await?;
            log!(
                "signaling state: {:?}",
                pc.signaling_state()
            );

            let sdp_js =
                Reflect::get(&answer, &"sdp".into())?;
            let sdp_str = sdp_js.as_string().unwrap();
            let encoded = general_purpose::STANDARD_NO_PAD
                .encode(sdp_str);
            let local_sdp_input_el =
                local_sdp_ref.get().unwrap();
            local_sdp_input_el.set_value(&encoded);
        }
        _ => {}
    }

    let candidates = &splitted[1..];
    candidates.iter().for_each(|candidate| {
        let splitted: Vec<&str> =
            candidate.split('=').collect();

        let v_candidate_encoded = splitted[0];
        let v_candidate_utf8 = general_purpose::STANDARD_NO_PAD
            .decode(v_candidate_encoded)
            .unwrap();
        let v_candidate = String::from_utf8(v_candidate_utf8).unwrap();
        let v_sdp_mid = splitted[1];
        let v_sdp_m_line_index =
            splitted[2].parse::<u16>().unwrap();

        let mut ice_candidate =
            RtcIceCandidateInit::new("");
        ice_candidate.candidate(&v_candidate);
        ice_candidate.sdp_mid(Some(v_sdp_mid));
        ice_candidate
            .sdp_m_line_index(Some(v_sdp_m_line_index));

        let _ = pc.add_ice_candidate_with_opt_rtc_ice_candidate_init(
            Some(&ice_candidate),
        );
    });

    Ok(())
}

pub(crate) fn track_ice_candidate_event(
    pc: &RtcPeerConnection,
    rtc_pc: leptos::ReadSignal<Option<RtcPeerConnection>>,
    local_sdp_ref: NodeRef<Textarea>,
) -> Result<(), JsValue> {
    let on_ice_candidate_callback =
        Closure::<dyn FnMut(_)>::new(
            move |ev: RtcPeerConnectionIceEvent| match ev
                .candidate()
            {
                Some(candidate) => {
                    let candidate_obj =
                        Object::unchecked_from_js(
                            JsValue::from(&candidate),
                        );

                    let v_candidate = Reflect::get(
                        &candidate_obj,
                        &"candidate".into(),
                    )
                    .unwrap()
                    .as_string()
                    .unwrap();
                    let v_candidate_encoded =
                        general_purpose::STANDARD_NO_PAD
                            .encode(v_candidate);
                    let v_sdp_mid = Reflect::get(
                        &candidate_obj,
                        &"sdpMid".into(),
                    )
                    .unwrap()
                    .as_string()
                    .unwrap();
                    let v_sdp_m_line_index = Reflect::get(
                        &candidate_obj,
                        &"sdpMLineIndex".into(),
                    )
                    .unwrap()
                    .as_f64()
                    .unwrap()
                        as u16;
                    let concated_candidate = format!(
                        "+{v_candidate_encoded}={v_sdp_mid}={v_sdp_m_line_index}",
                    );

                    let local_sdp_input_el =
                        local_sdp_ref.get().unwrap();
                    let local_sdp =
                        local_sdp_input_el.value();
                    let local_sdp_with_candidate = format!(
                        "{local_sdp}{concated_candidate}",
                    );
                    local_sdp_input_el.set_value(
                        &local_sdp_with_candidate,
                    );
                }
                None => {
                    log!("ICE gathering completed");
                }
            },
        );
    let on_ice_connection_state_change_callback =
        Closure::<dyn FnMut(_)>::new(
            move |ev: RtcIceConnectionState| {
                log!("ICE connection state: {:?}", &ev);

                let pc = match rtc_pc.get() {
                    Some(cn) => cn,
                    None => {
                        log!("no connection found");
                        return;
                    }
                };
                log!(
                    "ice connection state: {:?}",
                    pc.ice_connection_state()
                );
            },
        );

    pc.set_onicecandidate(Some(
        on_ice_candidate_callback.as_ref().unchecked_ref(),
    ));
    pc.set_oniceconnectionstatechange(Some(
        on_ice_connection_state_change_callback
            .as_ref()
            .unchecked_ref(),
    ));

    on_ice_candidate_callback.forget();
    on_ice_connection_state_change_callback.forget();

    Ok(())
}

async fn track_local_stream(
    pc: &RtcPeerConnection,
    local_stream_ref: NodeRef<Video>,
) -> Result<(), JsValue> {
    let mut media_constraints =
        MediaStreamConstraints::new();
    let ideal_constraint = Object::new();
    // TODO: this ideal type doesn't work on wasm api
    Reflect::set(
        &ideal_constraint,
        &"ideal".into(),
        &JsValue::from_bool(true),
    )?;
    media_constraints
        .video(&ideal_constraint)
        .audio(&ideal_constraint);

    let navigator = get_navigator()?;
    let media_devices = navigator.media_devices()?;
    let devices_enum =
        JsFuture::from(media_devices.enumerate_devices()?)
            .await?;
    log!(
        "devices: {:?}",
        Array::unchecked_from_js(
            Object::entries(Object::unchecked_from_js_ref(
                &devices_enum
            ))
            .get(0)
        )
    );

    let local_stream = MediaStream::from(
        JsFuture::from(
            media_devices.get_user_media_with_constraints(
                &media_constraints,
            )?,
        )
        .await?,
    );

    local_stream.get_tracks().for_each(
        &mut |track: JsValue, _, _| {
            let track = track.dyn_into().unwrap();
            pc.add_track_0(&track, &local_stream);
            log!("local track added.");

            if track.kind() == "video" {
                let video_track = Array::new();
                let _ = video_track.push(&track);

                let stream = MediaStream::new_with_tracks(
                    &video_track.into(),
                )
                .unwrap();

                let local_stream_el =
                    local_stream_ref.get().expect(
                        "cant find local stream element",
                    );
                local_stream_el
                    .set_src_object(Some(&stream));
            }
        },
    );
    Ok(())
}

fn track_remote_stream(
    pc: &RtcPeerConnection,
    remote_stream_ref: NodeRef<Video>,
) -> Result<(), JsValue> {
    let ontrack_callback = Closure::<dyn FnMut(_)>::new(
        move |ev: RtcTrackEvent| {
            log!("remote stream received.");
            let remote_video_track = ev.streams().get(0);
            let stream = MediaStream::new_with_tracks(
                &remote_video_track.dyn_into().unwrap(),
            )
            .unwrap();

            let remote_stream_el = remote_stream_ref
                .get()
                .expect("cant find local stream element");
            remote_stream_el.set_src_object(Some(&stream));
        },
    );

    pc.set_ontrack(Some(
        ontrack_callback.as_ref().unchecked_ref(),
    ));

    ontrack_callback.forget();

    Ok(())
}

pub(crate) async fn track_display_stream(
) -> Result<(), JsValue> {
    let navigator = get_navigator()?;

    let use_agent = navigator.user_agent()?;
    log!("user agent: {:?}", use_agent);

    let media_devices = navigator.media_devices()?;
    let display_devices =
        JsFuture::from(media_devices.get_display_media()?)
            .await?;
    let display_stream = MediaStream::from(display_devices);
    log!("display stream: {:?}", display_stream);

    Ok(())
}

fn get_navigator() -> Result<web_sys::Navigator, JsValue> {
    let window =
        window().ok_or(JsValue::from("no window found"))?;
    let is_secure = window.is_secure_context();
    log!("is secure: {:?}", is_secure);
    let navigator = window.navigator();

    Ok(navigator)
}
