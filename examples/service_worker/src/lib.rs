//! A simple example demonstrating how to integrate service worker into seed.
//! This example will cover the following:
//! 1. Cache resources
//! 2. Register the service worker
//! 3. If the service worker is not yet activated, an even listener will be registered, waiting for the
//!    state to reach "activated".
//! 4. When the state reaches "activated", the Notification object will request permission for notifications
//! 5. If permission is granted, the PushManager will subscribe using an example vapid key
//! 6. Finally, a PushSubscription will be returned, containing the information that can be passed to a
//!    notifcation back-end server.

pub mod error;

use crate::error::ServiceWorkerError;
use seed::{prelude::*, *};
use wasm_bindgen_futures::spawn_local;

// url_b64_to_uint_8_array - Takes a base_64_string and converts it to a js_sys::Uint8Array. This will be used
// to create an encoded key that will be used to subscribe to the push manager.
fn url_b64_to_uint_8_array(base_64_string: &str) -> Result<js_sys::Uint8Array, ServiceWorkerError> {
    let padding = std::iter::repeat('=')
        .take((4 - (base_64_string.len() % 4)) % 4)
        .collect::<String>();
    let base64 = format!("{}{}", base_64_string, &padding)
        .replace('-', "+")
        .replace('_', "/");
    let window = web_sys::window().ok_or(ServiceWorkerError::GetWindow)?;

    let raw_data: String = window.atob(&base64).map_err(ServiceWorkerError::MapAToB)?;
    let output_array: js_sys::Uint8Array =
        js_sys::Uint8Array::new_with_length(raw_data.chars().count() as u32);

    let mut pos = 0;
    for c in raw_data.chars() {
        output_array.set_index(pos, c as u8);
        pos = pos + 1;
    }

    Ok(output_array)
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct PushSubscriptionKeys {
    p256dh: String,
    auth: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct PushSubscription {
    endpoint: String,
    expiration_time: Option<String>,
    keys: PushSubscriptionKeys,
}

// Register the push manager given a ServiceWorkerRegistration object
async fn register_push_manager(
    sw_reg: web_sys::ServiceWorkerRegistration,
    app: App<Msg, Model, Node<Msg>>,
) -> Result<(), ServiceWorkerError> {
    let permission = web_sys::Notification::request_permission()
        .map_err(ServiceWorkerError::RequestPermission)?;
    let permission = wasm_bindgen_futures::JsFuture::from(permission)
        .await
        .map_err(ServiceWorkerError::RequestPermission)?;
    let permission: String = JsValue::into_serde(&permission)?;

    if !permission.eq("granted".into()) {
        return Err(ServiceWorkerError::InvalidPermissions);
    }

    let manager: web_sys::PushManager = sw_reg
        .push_manager()
        .map_err(ServiceWorkerError::RetrievePushManager)?;

    // Using `web-push generate-vapid-keys` the following is generated for this example:
    // =======================================

    // Public Key:
    // BPUHCMCC6_WLIQh-eo0Bmh-w0fG5txRVLfjVfOXRcGVfIcQeaMSPAin0Q-WHgxNENK_2NCJykknLX7fKN9XY-QQ

    // Private Key:
    // F2iuoMyqIQKxCuinBuZKodP-6wnUcrW6tsHfbsxwSUA

    // =======================================
    let key =
        "BPUHCMCC6_WLIQh-eo0Bmh-w0fG5txRVLfjVfOXRcGVfIcQeaMSPAin0Q-WHgxNENK_2NCJykknLX7fKN9XY-QQ";
    let encoded_key = url_b64_to_uint_8_array(key)?;

    // In order to subscribe to PushNotifications we need to specify two things:
    // 1. The application server key
    // 2. userVisibleOnly MUST be set to true (this used to only apply to chrome but it appears firefox requires it as well).
    // As of Aug 29, 2020, wasm_bindgen does not currently provide the `userVisibleOnly` property. A PR was submitted that should
    // make its way into the 0.2.68 release: https://github.com/rustwasm/wasm-bindgen/commit/49dc58e58f0a8b5921eb7602ab72e82ec51e65e4
    let subscription: JsValue;
    unsafe {
        subscription = subscribe(manager.clone(), encoded_key).await;
    }

    let push_subscription: PushSubscription = subscription.into_serde()?;
    app.update(Msg::SubscriptionRetrieved(push_subscription));

    Ok(())
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    sw_reg: Option<web_sys::ServiceWorkerRegistration>,
    push_subscription: Option<PushSubscription>,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
enum Msg {
    SubscriptionRetrieved(PushSubscription),
    SendMessage,
    SetServiceWorker(web_sys::ServiceWorkerRegistration),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SubscriptionRetrieved(push_subscription) => {
            log!(
                "Got a push subscription of: {:?}",
                serde_json::to_string_pretty(&push_subscription).unwrap()
            );
            model.push_subscription = Some(push_subscription);
        }
        Msg::SendMessage => {
            web_sys::Notification::new("Hello from seed service worker!")
                .expect("Couldn't send notification.");
        }
        Msg::SetServiceWorker(sw_reg) => {
            model.sw_reg = Some(sw_reg);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::SendMessage), "Send Message"],
        h1!["Push Subscription"],
        code![serde_json::to_string_pretty(&model.push_subscription).unwrap()],
        br![],
        br![],
        img![attrs! {
            At::Src => "https://media.prod.mdn.mozit.cloud/attachments/2016/02/29/12630/a1182129bede4bea905f11c9ba475c17/important-notes.png"
        },],
        br![],
        a![
            attrs! {
                At::Href => "https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API/Using_Service_Workers"
            },
            "- Using Service Workers (Service Worker Api)"
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    let app = App::start("app", init, update, view);

    spawn_local(async {
        match register_service_worker(app).await {
            Ok(x) => x,
            Err(e) => {
                log!("Error registering service worker: {:?}", e);
            }
        }
    });
}

async fn register_service_worker(
    app: App<Msg, Model, Node<Msg>>,
) -> Result<(), ServiceWorkerError> {
    let window = web_sys::window().ok_or(ServiceWorkerError::GetWindow)?;
    let sw_container = window.navigator().service_worker();

    let p = sw_container.register("service-worker.js");
    let reg = wasm_bindgen_futures::JsFuture::from(p)
        .await
        .map_err(ServiceWorkerError::Registration)?;

    let sw_reg: web_sys::ServiceWorkerRegistration = reg.into();

    app.update(Msg::SetServiceWorker(sw_reg.clone()));
    let sw: Option<web_sys::ServiceWorker> = if let Some(x) = sw_reg.installing() {
        Some(x)
    } else if let Some(x) = sw_reg.waiting() {
        Some(x)
    } else if let Some(x) = sw_reg.active() {
        Some(x)
    } else {
        None
    };

    if let Some(sw) = sw {
        if sw.state() == web_sys::ServiceWorkerState::Activated {
            log!("Service worker is activated! About to register with push manager.");
            register_push_manager(sw_reg.clone(), app.clone()).await?;
        } else {
            log!("Service worker not activated. Registering an event listener and waiting.");
            let c = Closure::wrap(Box::new(move |e: web_sys::Event| {
                let target: web_sys::EventTarget = e.target().expect("Couldn't get target");
                let sw_js: JsValue = target.into();
                let sw: web_sys::ServiceWorker = sw_js.into();
                let state: web_sys::ServiceWorkerState = sw.state();

                if state == web_sys::ServiceWorkerState::Activated {
                    let f = register_push_manager(sw_reg.clone(), app.clone());
                    spawn_local(async {
                        match f.await {
                            Ok(x) => x,
                            Err(e) => log!(
                                "Error encountered while registering the Push Manager: {:?}",
                                e
                            ),
                        }
                    });
                }
            }) as Box<dyn Fn(web_sys::Event)>);

            sw.add_event_listener_with_callback("statechange", &c.as_ref().unchecked_ref())
                .map_err(|v| {
                    sw.remove_event_listener_with_callback(
                        "statechange",
                        &c.as_ref().unchecked_ref(),
                    )
                    .expect("Couldn't remove state change event listener.");
                    ServiceWorkerError::StateChangeListener(v)
                })?;

            c.forget();
        }
    } else {
        return Err(ServiceWorkerError::InvalidState);
    }

    Ok(())
}

#[wasm_bindgen(module = "/custom.js")]
#[rustfmt::skip]
extern "C" {
    async fn subscribe(
        manager: web_sys::PushManager,
        api_key: js_sys::Uint8Array,
    ) -> wasm_bindgen::JsValue;
}
