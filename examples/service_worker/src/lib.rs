//! A simple example demonstrating how to integrate service worker into seed.
//! This example will cover the following:
//! 1. Cache resources
//! 2. Register the service worker
//! 3. If the service worker is not yet activated, an even listener will be registered, waiting for the
//!    state to reach "activated".
//! 4. When the state reaches "activated", the Notification object will request permission for notifications
//! 5. If permission is granted, the `PushManager` will subscribe using an example vapid key
//! 6. Finally, a `PushSubscription` will be returned, containing the information that can be passed to a
//!    notifcation back-end server.

#![allow(clippy::cast_possible_truncation, clippy::needless_pass_by_value)]

pub mod errors;

use crate::errors::ServiceWorkerError;
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

    for (pos, c) in raw_data.chars().enumerate() {
        output_array.set_index(pos as u32, c as u8);
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
) -> Result<Msg, ServiceWorkerError> {
    let permission = web_sys::Notification::request_permission()
        .map_err(ServiceWorkerError::RequestPermission)?;
    let permission = wasm_bindgen_futures::JsFuture::from(permission)
        .await
        .map_err(ServiceWorkerError::RequestPermission)?;
    let permission: String = JsValue::into_serde(&permission)?;

    if !permission.eq("granted") {
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
    Ok(Msg::SubscriptionRetrieved(push_subscription))
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
    activated: bool,
    error: Option<String>,
    message: Option<String>,
    notifications_granted: bool,
    push_subscription: Option<PushSubscription>,
    sw_reg: Option<web_sys::ServiceWorkerRegistration>,
}

// ------ ------
//    Update
// ------ ------

enum ServiceWorkerStatus {
    Activated,
    Waiting,
    StateChange(web_sys::ServiceWorkerState),
    SubscriptionRetrieved,
}

enum Msg {
    FailedToRegisterPushManager(ServiceWorkerError),
    FailedToRegisterServiceWorker(ServiceWorkerError),
    RequestNotificationPermission,
    ServiceWorkerActivated,
    SubscriptionRetrieved(PushSubscription),
    SendMessage,
    SetServiceWorker(web_sys::ServiceWorkerRegistration),
    StatusUpdate(ServiceWorkerStatus),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FailedToRegisterPushManager(e) => {
            log!("Error encountered while registering the Push Manager: ", e);
            model.error = Some(format!(
                "Error encountered while registering the Push Manager: {:?}",
                e
            ));
        }
        Msg::FailedToRegisterServiceWorker(e) => {
            log!("Failed to register service worker: ", e);
            model.error = Some(format!("Failed to register service worker: {:?}", e));
        }
        Msg::StatusUpdate(status) => match status {
            ServiceWorkerStatus::Activated => {
                model.message = Some("Service worker is activated!".into())
            }
            ServiceWorkerStatus::Waiting => {
                model.message = Some(
                    "Service worker not activated. Registering an event listener and waiting."
                        .into(),
                )
            }
            ServiceWorkerStatus::StateChange(state) => {
                model.message = Some(format!("Service worker state changed to: {:?}", state))
            }
            ServiceWorkerStatus::SubscriptionRetrieved => {
                model.message = Some("PushManager Subscription retrieved.".into())
            }
        },
        Msg::RequestNotificationPermission => {
            if let Some(sw_reg) = model.sw_reg.clone() {
                orders.perform_cmd(async move {
                    match register_push_manager(sw_reg.clone()).await {
                        Ok(msg) => msg,
                        Err(e) => Msg::FailedToRegisterPushManager(e),
                    }
                });
            }
        }
        Msg::ServiceWorkerActivated => {
            model.activated = true;

            let permission: web_sys::NotificationPermission = web_sys::Notification::permission();
            if permission == web_sys::NotificationPermission::Granted {
                model.notifications_granted = true;
            }
        }
        Msg::SubscriptionRetrieved(push_subscription) => {
            log!(
                "Got a push subscription of:",
                serde_json::to_string_pretty(&push_subscription).unwrap()
            );
            model.notifications_granted = true;
            model.push_subscription = Some(push_subscription);
            orders.send_msg(Msg::StatusUpdate(
                ServiceWorkerStatus::SubscriptionRetrieved,
            ));
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
        h1!["Seed - Service Worker Demo"],
        p!["When the page first loads, the service worker will cache all assets, but permissions for notifications must be granted."],
        ol![
            li![
                "Click the ",
                b!["Request Push Subscription"],
                " button and when prompted, select ",
                b!["Allow Notifications."],
            ],
            li!["The subscription information will be printed to the page and notifications will now be granted."],
            li![
                "Click the ",
                b!["Send Message"],
                " button to see a browser notification coming from service worker.",
            ],
        ],
        button![ev(Ev::Click, |_| Msg::RequestNotificationPermission), "Request Push Subscription"],
        button![
            ev(Ev::Click, |_| Msg::SendMessage),
            IF!(not(model.notifications_granted) => attrs!{At::Disabled => true}),
            "Send Message"
        ],
        IF!(model.error.is_some() => {
            p![
                attrs!{At::Style => "color:red"},
                model.error.clone().unwrap()
            ]
        }),
        IF!(model.message.is_some() => {
            p![
                model.message.clone().unwrap()
            ]
        }),
        h2!["Push Subscription"],
        code![serde_json::to_string_pretty(&model.push_subscription).unwrap()],
        br![],
        br![],
        img![attrs! {
            At::Src => "images/important-notes.png"
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

    spawn_local(async move {
        match register_service_worker(app.clone()).await {
            Ok(x) => x,
            Err(e) => {
                app.update(Msg::FailedToRegisterServiceWorker(e));
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
            app.update(Msg::ServiceWorkerActivated);
            app.update(Msg::StatusUpdate(ServiceWorkerStatus::Activated));
        } else {
            app.update(Msg::StatusUpdate(ServiceWorkerStatus::Waiting));
            let c = Closure::wrap(Box::new(move |e: web_sys::Event| {
                let target: web_sys::EventTarget = e.target().expect("Couldn't get target");
                let sw_js: JsValue = target.into();
                let sw: web_sys::ServiceWorker = sw_js.into();
                let state: web_sys::ServiceWorkerState = sw.state();

                app.update(Msg::StatusUpdate(ServiceWorkerStatus::StateChange(state)));

                if state == web_sys::ServiceWorkerState::Activated {
                    app.update(Msg::ServiceWorkerActivated);
                }
            }) as Box<dyn Fn(web_sys::Event)>);

            sw.add_event_listener_with_callback("statechange", c.as_ref().unchecked_ref())
                .map_err(|v| {
                    sw.remove_event_listener_with_callback(
                        "statechange",
                        c.as_ref().unchecked_ref(),
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
