use base64::engine::general_purpose;
use base64::Engine;
use js_sys::{Array, Object, Reflect};
use leptos::{log, SignalSet, WriteSignal};
use wasm_bindgen::{
    convert::IntoWasmAbi, prelude::Closure, JsCast, JsValue,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    RtcConfiguration, RtcPeerConnection,
    RtcPeerConnectionIceEvent, RtcSdpType,
    RtcSessionDescriptionInit,
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
    set_local_sdp: WriteSignal<String>,
) -> Result<(), JsValue> {
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

    // let encoded = general_purpose::STANDARD_NO_PAD.encode();
    // set_local_sdp
    //     .set(JsValue::from(offer).unchecked_into());

    Ok(())
}

pub async fn answer_offer(
    session: crate::landing_page::Session,
    pc: &RtcPeerConnection,
) -> Result<(), JsValue> {
    let mut remote_offer =
        RtcSessionDescriptionInit::new(RtcSdpType::Offer);
    remote_offer.sdp(&session.sdp);
    log!("remote description: {:?}", &remote_offer);

    let _ = JsFuture::from(
        pc.set_remote_description(&remote_offer),
    )
    .await?;
    log!("signaling state: {:?}", pc.signaling_state());

    let answer = JsFuture::from(pc.create_answer()).await?;
    let answer = RtcSessionDescriptionInit::from(answer);
    log!("answer: {:?}", &answer);
    let _ =
        JsFuture::from(pc.set_local_description(&answer))
            .await?;
    log!("signaling state: {:?}", pc.signaling_state());

    Ok(())
}

pub fn on_ice_candidate(
    pc: &RtcPeerConnection,
) -> Result<(), JsValue> {
    let pc_clone = pc.clone();
    let on_ice_candidate_callback =
        Closure::<dyn FnMut(_)>::new(
            move |ev: RtcPeerConnectionIceEvent| {
                if let Some(candidate) = ev.candidate() {
                    log!("ICE candidate: {:?}", candidate);
                }
            },
        );
    let on_ice_connection_state_change_callback =
        Closure::<dyn FnMut(_)>::new(
            move |_: RtcPeerConnectionIceEvent| {
                log!(
                    "ICE connection state: {:?}",
                    pc_clone.ice_connection_state()
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

pub fn handle_events(
    pc: &RtcPeerConnection,
    event: String,
) {
    match event.as_str() {
        "icecandidate" => {
            let on_ice_candidate =
                Box::new(move |event: JsValue| {
                    let candidate = Reflect::get(
                        &event,
                        &"candidate".into(),
                    )
                    .unwrap()
                    .as_string()
                    .unwrap();

                    println!(
                        "ICE candidate: {}",
                        candidate
                    );
                });
        }
        _ => {
            println!("Unhandled event: {}", event);
        }
    };
}
