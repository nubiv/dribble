use base64::engine::general_purpose;
use base64::Engine;
use js_sys::{Array, Object, Reflect};
use leptos::{
    html::{Input, Textarea, Video},
    log, NodeRef,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, MediaStream, MediaStreamConstraints,
    RtcConfiguration, RtcPeerConnection,
    RtcPeerConnectionIceEvent, RtcSdpType,
    RtcSessionDescriptionInit, RtcSignalingState,
    RtcTrackEvent,
};

pub fn init_connection(
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
        rtc_configuration
    })
}

pub async fn create_offer(
    pc: &RtcPeerConnection,
    local_stream_ref: NodeRef<Video>,
    remote_stream_ref: NodeRef<Video>,
    local_sdp_ref: NodeRef<Textarea>,
) -> Result<(), JsValue> {
    on_ice_candidate(pc).unwrap();
    track_local_stream(pc, local_stream_ref).await?;
    trach_remote_stream(pc, remote_stream_ref)?;

    let local_offer =
        JsFuture::from(pc.create_offer()).await?;
    let local_offer =
        RtcSessionDescriptionInit::from(local_offer);
    log!("local description: {:?}", &local_offer);

    let _ = JsFuture::from(
        pc.set_local_description(&local_offer),
    )
    .await?;
    log!("signaling state: {:?}", pc.signaling_state());

    let sdp_js = Reflect::get(&local_offer, &"sdp".into())?;
    let sdp_str = sdp_js.as_string().unwrap();
    log!("sdp_str: {:?}", sdp_str);
    let encoded =
        general_purpose::STANDARD_NO_PAD.encode(sdp_str);
    let local_sdp_input_el = local_sdp_ref.get().unwrap();
    local_sdp_input_el.set_value(&encoded);

    Ok(())
}

pub async fn answer_offer(
    remote_sdp_encoded: &str,
    pc: &RtcPeerConnection,
    local_stream_ref: NodeRef<Video>,
    remote_stream_ref: NodeRef<Video>,
    local_sdp_ref: NodeRef<Textarea>,
) -> Result<(), JsValue> {
    let mut remote_offer =
        RtcSessionDescriptionInit::new(RtcSdpType::Offer);
    remote_offer.sdp(remote_sdp_encoded);
    log!("remote description: {:?}", &remote_offer);

    let _ = JsFuture::from(
        pc.set_remote_description(&remote_offer),
    )
    .await?;
    log!("signaling state: {:?}", pc.signaling_state());

    if pc.signaling_state()
        == RtcSignalingState::HaveRemoteOffer
    {
        on_ice_candidate(pc).unwrap();
        track_local_stream(pc, local_stream_ref).await?;
        trach_remote_stream(pc, remote_stream_ref)?;

        let answer =
            JsFuture::from(pc.create_answer()).await?;
        let answer =
            RtcSessionDescriptionInit::from(answer);
        log!("answer: {:?}", &answer);
        let _ = JsFuture::from(
            pc.set_local_description(&answer),
        )
        .await?;
        log!("signaling state: {:?}", pc.signaling_state());

        let sdp_js = Reflect::get(&answer, &"sdp".into())?;
        let sdp_str = sdp_js.as_string().unwrap();
        log!("sdp_str: {:?}", sdp_str);
        let encoded = general_purpose::STANDARD_NO_PAD
            .encode(sdp_str);
        let local_sdp_input_el =
            local_sdp_ref.get().unwrap();
        local_sdp_input_el.set_value(&encoded);
    }

    Ok(())
}

pub fn on_ice_candidate(
    pc: &RtcPeerConnection,
) -> Result<(), JsValue> {
    let on_ice_candidate_callback =
        Closure::<dyn FnMut(_)>::new(
            move |ev: RtcPeerConnectionIceEvent| {
                if let Some(candidate) = ev.candidate() {
                    log!(
                        "ICE candidate: {:?}",
                        Object::entries(&candidate)
                    );
                }
            },
        );
    let on_ice_connection_state_change_callback =
        Closure::<dyn FnMut(_)>::new(
            move |ev: RtcPeerConnectionIceEvent| {
                log!(
                    "ICE connection state: {:?}",
                    Object::entries(&ev)
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
    // TODO: this ideal type doesn't seem to work
    Reflect::set(
        &ideal_constraint,
        &"ideal".into(),
        &JsValue::from_bool(true),
    )?;
    media_constraints
        .video(&ideal_constraint)
        .audio(&ideal_constraint);

    let window = window().unwrap();
    let is_secure = window.is_secure_context();
    log!("is secure: {:?}", is_secure);
    let navigator = window.navigator();
    let media_devices = navigator.media_devices()?;

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
            log!("added local track to pc.");

            if track.kind() == "video" {
                let tracks = Array::new();
                let _ = tracks.push(&track);

                let stream = MediaStream::new_with_tracks(
                    &tracks.into(),
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

fn trach_remote_stream(
    pc: &RtcPeerConnection,
    remote_stream_ref: NodeRef<Video>,
) -> Result<(), JsValue> {
    let ontrack_callback = Closure::<dyn FnMut(_)>::new(
        move |ev: RtcTrackEvent| {
            let remote_stream = ev.streams().at(0);
            let stream = MediaStream::new_with_tracks(
                &remote_stream,
            )
            .unwrap();
            log!("added remote stream.");

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
