//! An example demonstrating how to integrate service worker into Seed.
//! This example will cover the following:
//! 1. Cache resources
//! 2. Register the service worker
//! 3. If the service worker is not yet activated, an even listener will be registered, waiting for the
//!    state to reach "activated".
//! 4. When the state reaches "activated", the Notification object will request permission for notifications
//! 5. If permission is granted, the `PushManager` will subscribe using an example vapid key
//! 6. Finally, a `PushSubscription` will be returned, containing the information that can be passed to a
//!    notification back-end server.
// @TODO: Replace the call to `subscribe` with the web_sys equivalent when wasm_bindgen releases a version >= 0.2.68.

#![allow(clippy::needless_pass_by_value)]

pub mod errors;

use crate::errors::ServiceWorkerError;
use futures::future::try_join_all;
use seed::{prelude::*, *};
use std::rc::Rc;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    // Get a message sender that can be passed to the register_service_worker function.
    let msg_sender = orders.msg_sender();

    orders.perform_cmd(async move {
        match register_service_worker(Rc::clone(&msg_sender)).await {
            Ok(x) => x,
            Err(e) => msg_sender(Some(Msg::FailedToRegisterServiceWorker(e))),
        }
    });

    Model::default()
}

/// Register the service worker and send status update messages.
async fn register_service_worker(
    msg_sender: Rc<dyn Fn(Option<Msg>)>,
) -> Result<(), ServiceWorkerError> {
    let window = web_sys::window().ok_or(ServiceWorkerError::GetWindow)?;
    let sw_container = window.navigator().service_worker();

    let p = sw_container.register("service-worker.js");
    let reg = wasm_bindgen_futures::JsFuture::from(p)
        .await
        .map_err(ServiceWorkerError::Registration)?;

    let sw_reg: web_sys::ServiceWorkerRegistration = reg.into();

    msg_sender(Some(Msg::SetServiceWorker(sw_reg.clone())));
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
            msg_sender(Some(Msg::ServiceWorkerActivated));
            msg_sender(Some(Msg::StatusUpdate(ServiceWorkerStatus::Activated)));
        } else {
            msg_sender(Some(Msg::StatusUpdate(ServiceWorkerStatus::Waiting)));
            let c = Closure::wrap(Box::new(move |e: web_sys::Event| {
                let target: web_sys::EventTarget = e.target().expect("Couldn't get target");
                let sw_js: JsValue = target.into();
                let sw: web_sys::ServiceWorker = sw_js.into();
                let state: web_sys::ServiceWorkerState = sw.state();

                msg_sender(Some(Msg::StatusUpdate(ServiceWorkerStatus::StateChange(
                    state,
                ))));

                if state == web_sys::ServiceWorkerState::Activated {
                    msg_sender(Some(Msg::ServiceWorkerActivated));
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

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct PushSubscription {
    endpoint: String,
    expiration_time: Option<String>,
    keys: PushSubscriptionKeys,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct PushSubscriptionKeys {
    p256dh: String,
    auth: String,
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
    CacheCleared,
    FailedToClearCache(ServiceWorkerError),
    ClearCache,
    FailedToRegisterPushManager(ServiceWorkerError),
    FailedToRegisterServiceWorker(ServiceWorkerError),
    FailedToUnregisterServiceWorker(ServiceWorkerError),
    RequestNotificationPermission,
    ServiceWorkerActivated,
    SubscriptionRetrieved(PushSubscription),
    SendMessage,
    SetServiceWorker(web_sys::ServiceWorkerRegistration),
    StatusUpdate(ServiceWorkerStatus),
    UnregisterServiceWorker,
    UnregisteredServiceWorker,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CacheCleared => {
            model.message = Some("Cache cleared.".into());
        }
        Msg::FailedToClearCache(e) => {
            model.error = Some(format!("{:?}", e));
        }
        Msg::ClearCache => {
            orders.perform_cmd(async move {
                match clear_cache().await {
                    Ok(_) => Msg::CacheCleared,
                    Err(e) => Msg::FailedToClearCache(e),
                }
            });
        }
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
        Msg::FailedToUnregisterServiceWorker(e) => {
            log!("Failed to unregister service worker: ", e);
            model.error = Some(format!("Failed to unregister service worker: {:?}", e));
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
            web_sys::Notification::new("Hello from Seed service worker!")
                .expect("Couldn't send notification.");
        }
        Msg::SetServiceWorker(sw_reg) => {
            model.sw_reg = Some(sw_reg);
        }
        Msg::UnregisterServiceWorker => {
            orders.perform_cmd(async move {
                match unregister_service_worker().await {
                    Ok(_) => Msg::UnregisteredServiceWorker,
                    Err(e) => Msg::FailedToUnregisterServiceWorker(e),
                }
            });
        }
        Msg::UnregisteredServiceWorker => {
            model.message = Some("Successfully unregistered the service worker.".into());
            model.activated = false;
        }
    }
}

/// Loop through each of the caches and delete each one.
async fn clear_cache() -> Result<(), ServiceWorkerError> {
    let window = web_sys::window().ok_or(ServiceWorkerError::GetWindow)?;
    let caches: web_sys::CacheStorage = window.caches().map_err(ServiceWorkerError::GetCaches)?;
    let keys: JsValue = wasm_bindgen_futures::JsFuture::from(caches.keys())
        .await
        .map_err(ServiceWorkerError::GetCacheKeys)?;
    let cache_names: Vec<String> = JsValue::into_serde(&keys)?;

    let futures = cache_names
        .into_iter()
        .map(|name| wasm_bindgen_futures::JsFuture::from(caches.delete(&name)));

    try_join_all(futures)
        .await
        .map_err(ServiceWorkerError::GetCacheKeys)?;

    Ok(())
}

/// Unregister the service worker. This will be called when a user clicks on the corresponding button.
async fn unregister_service_worker() -> Result<(), ServiceWorkerError> {
    let window = web_sys::window().ok_or(ServiceWorkerError::GetWindow)?;
    let sw_container: web_sys::ServiceWorkerContainer = window.navigator().service_worker();

    let registration: JsValue =
        wasm_bindgen_futures::JsFuture::from(sw_container.get_registration())
            .await
            .map_err(ServiceWorkerError::GetServiceWorkerRegistration)?;
    let registration: web_sys::ServiceWorkerRegistration = registration.into();

    let f = registration
        .unregister()
        .map_err(ServiceWorkerError::UnregisterServiceWorker)?;

    wasm_bindgen_futures::JsFuture::from(f)
        .await
        .map_err(ServiceWorkerError::UnregisterServiceWorker)?;

    Ok(())
}

/// Register the push manager given a `ServiceWorkerRegistration` object.
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

    // In order to subscribe to PushNotifications we need to specify two things:
    // 1. The application server key
    // 2. userVisibleOnly MUST be set to true (this used to only apply to chrome but it appears firefox requires it as well).
    // As of Aug 29, 2020, wasm_bindgen does not currently provide the `userVisibleOnly` property. A PR was submitted that should
    // make its way into the 0.2.68 release: https://github.com/rustwasm/wasm-bindgen/commit/49dc58e58f0a8b5921eb7602ab72e82ec51e65e4
    let subscription: JsValue;
    subscription = subscribe(&manager, key).await;

    let push_subscription: PushSubscription = subscription.into_serde()?;
    Ok(Msg::SubscriptionRetrieved(push_subscription))
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
        button![
            ev(Ev::Click, |_| Msg::RequestNotificationPermission),
            attrs!{At::Disabled => (!model.activated).as_at_value()},
            "Request Push Subscription"
        ],
        button![
            ev(Ev::Click, |_| Msg::SendMessage),
            attrs!{At::Disabled => (!model.notifications_granted).as_at_value()},
            "Send Message"
        ],
        button![
            ev(Ev::Click, |_| Msg::ClearCache),
            "Clear Cache"
        ],
        button![
            ev(Ev::Click, |_| Msg::UnregisterServiceWorker),
            attrs!{At::Disabled => (!model.activated).as_at_value()},
            "Unregister Service Worker"
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
    App::start("app", init, update, view);
}

#[wasm_bindgen]
// @TODO: Remove the line below once https://github.com/rust-lang/rustfmt/issues/4288 is resolved
// and a new `rustfmt` version is released.
#[rustfmt::skip]
extern "C" {
    async fn subscribe(
        manager: &web_sys::PushManager,
        api_key: &str,
    ) -> JsValue;
}
