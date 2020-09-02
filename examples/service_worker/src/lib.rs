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

#![allow(clippy::needless_pass_by_value)]

use apply::Apply;
use futures::future::try_join_all;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    let worker_container = window().navigator().service_worker();
    let ready = worker_container
        .ready()
        .map(JsFuture::from)
        .expect("get `ready`");

    orders
        .perform_cmd(async move {
            Msg::WorkerActivated({
                ready
                    .await
                    .map(web_sys::ServiceWorkerRegistration::from)
                    .expect("ServiceWorkerRegistration on ready")
            })
        })
        .perform_cmd(async move {
            worker_container
                .register("/service-worker.js")
                .apply(JsFuture::from)
                .await
                .map_err(ServiceWorkerError::WorkerRegistration)
                .err()
                .map(Msg::WorkerRegistrationFailed)
        });

    Model {
        error: None,
        message: None,
        worker_data: None,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    error: Option<String>,
    message: Option<String>,
    worker_data: Option<WorkerData>,
}

struct WorkerData {
    notifications_granted: bool,
    push_subscription: Option<PushSubscription>,
    registration: web_sys::ServiceWorkerRegistration,
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

enum Msg {
    WorkerActivated(web_sys::ServiceWorkerRegistration),
    SubscriptionReceived(Option<PushSubscription>),
    WorkerRegistrationFailed(ServiceWorkerError),
    UnregisterWorker,
    WorkerUnregistered(Option<ServiceWorkerError>),
    RequestNotificationPermission,
    NotificationPermissionRequested(Result<web_sys::NotificationPermission, ServiceWorkerError>),
    Subscribed(Result<PushSubscription, ServiceWorkerError>),
    SendMessage,
    ClearCache,
    CacheCleared(Option<ServiceWorkerError>),
}

#[derive(Debug)]
pub enum ServiceWorkerError {
    DeleteCache(JsValue),
    InvalidPermissions,
    WorkerRegistration(JsValue),
    RequestPermission(JsValue),
    SerdeJson(serde_json::error::Error),
    WorkerUnregistration(Option<JsValue>),
}

#[allow(clippy::too_many_lines, clippy::match_same_arms)]
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::WorkerActivated(worker_registration) => {
            log!("Worker activated.");
            model.message = Some("Service worker activated!".to_owned());

            let push_manager = worker_registration
                .push_manager()
                .expect("get `PushManager`");

            if let Ok(get_subscription) = push_manager.get_subscription().map(JsFuture::from) {
                orders.perform_cmd(async move {
                    Msg::SubscriptionReceived({
                        get_subscription
                            .await
                            .ok()
                            .and_then(|subscription| subscription.into_serde().ok())
                    })
                });
            };

            model.worker_data = Some(WorkerData {
                notifications_granted: web_sys::NotificationPermission::Granted
                    == web_sys::Notification::permission(),
                push_subscription: None,
                registration: worker_registration,
            });
        }
        Msg::SubscriptionReceived(subscription) => {
            let worker_data = match &mut model.worker_data {
                None => return,
                Some(worker_data) => worker_data,
            };
            worker_data.push_subscription = subscription;
        }
        Msg::WorkerRegistrationFailed(worker_error) => {
            model.error = Some(log_error(&worker_error));
        }
        Msg::UnregisterWorker => {
            let worker_data = match model.worker_data.take() {
                None => return,
                Some(worker_data) => worker_data,
            };

            let unregister = worker_data
                .registration
                .unregister()
                .map(JsFuture::from)
                .expect("get `unregister`");

            orders.perform_cmd(async move {
                Msg::WorkerUnregistered(
                    match unregister
                        .await
                        .map(|js_value| js_value.as_bool().expect("`bool` from `JsValue`"))
                    {
                        Ok(true) => None,
                        Ok(false) => Some(ServiceWorkerError::WorkerUnregistration(None)),
                        Err(error) => Some(ServiceWorkerError::WorkerUnregistration(Some(error))),
                    },
                )
            });
        }
        Msg::WorkerUnregistered(None) => {
            model.message = Some("Successfully unregistered the service worker.".to_owned());
        }
        Msg::WorkerUnregistered(Some(worker_error)) => {
            model.error = Some(log_error(&worker_error));
        }
        Msg::RequestNotificationPermission => {
            orders.perform_cmd(async {
                Msg::NotificationPermissionRequested({
                    web_sys::Notification::request_permission()
                        .expect("call `request_permission`")
                        .apply(JsFuture::from)
                        .await
                        .map(|js_value| {
                            web_sys::NotificationPermission::from_js_value(&js_value)
                                .expect("`NotificationPermission` from `JsValue`")
                        })
                        .map_err(ServiceWorkerError::RequestPermission)
                })
            });
        }
        Msg::NotificationPermissionRequested(Ok(web_sys::NotificationPermission::Granted)) => {
            let worker_data = match &mut model.worker_data {
                None => return,
                Some(worker_data) => worker_data,
            };

            worker_data.notifications_granted = true;

            // @TODO What is web-push and how to use it?
            // @TODO Do we want to include the link below somewhere?
            // https://medium.com/izettle-engineering/beginners-guide-to-web-push-notifications-using-service-workers-cb3474a17679

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
            let push_manager = worker_data
                .registration
                .push_manager()
                .expect("get `PushManager`");
            // @TODO: Replace the call to `subscribe` with the web_sys equivalent when wasm_bindgen releases a version >= 0.2.68.
            orders.perform_cmd(async move {
                Msg::Subscribed(
                    subscribe(&push_manager, key)
                        .await
                        .into_serde()
                        .map_err(ServiceWorkerError::SerdeJson),
                )
            });
        }
        Msg::NotificationPermissionRequested(Ok(_)) => {
            let worker_error = ServiceWorkerError::InvalidPermissions;
            model.error = Some(log_error(&worker_error));
        }
        Msg::NotificationPermissionRequested(Err(worker_error)) => {
            model.error = Some(log_error(&worker_error));
        }
        Msg::Subscribed(Ok(push_subscription)) => {
            let worker_data = match &mut model.worker_data {
                None => return,
                Some(worker_data) => worker_data,
            };

            log!("subscription:", push_subscription);
            worker_data.push_subscription = Some(push_subscription);
        }
        Msg::Subscribed(Err(worker_error)) => {
            model.error = Some(log_error(&worker_error));
        }
        Msg::SendMessage => {
            // @TODO: It doesn't use push - it's only a simple local browser notification.
            // @TODO: We should send a notification through subscription's endpoint (fetch?).
            // @TODO: Inspiration
            //    - https://serviceworke.rs/push-subscription-management_server_doc.html
            //    - https://medium.com/izettle-engineering/beginners-guide-to-web-push-notifications-using-service-workers-cb3474a17679
            // @TODO: Once fetch is ready, modify the condition in `view` to disable "Send message" button when the subscription isn't set.
            web_sys::Notification::new("Hello from Seed!").expect("send notification");
        }
        Msg::ClearCache => {
            orders.perform_cmd(async {
                Msg::CacheCleared(
                    async {
                        // Loop through each of the caches and delete each one.
                        let cache_storage = window().caches().expect("get `CacheStorage`");
                        let keys = JsFuture::from(cache_storage.keys())
                            .await
                            .expect("get cache keys");
                        let keys: Vec<String> =
                            JsValue::into_serde(&keys).map_err(ServiceWorkerError::SerdeJson)?;

                        let futures = keys
                            .into_iter()
                            .map(|key| JsFuture::from(cache_storage.delete(&key)));

                        try_join_all(futures)
                            .await
                            .map_err(ServiceWorkerError::DeleteCache)
                    }
                    .await
                    .err(),
                )
            });
        }
        Msg::CacheCleared(None) => {
            model.message = Some("Cache cleared.".to_owned());
        }
        Msg::CacheCleared(Some(worker_error)) => {
            model.error = Some(log_error(&worker_error));
        }
    }
}

fn log_error(worker_error: &ServiceWorkerError) -> String {
    let message = match worker_error {
        ServiceWorkerError::DeleteCache(_) => "Failed to delete cache.",
        ServiceWorkerError::InvalidPermissions => "User has not granted notification permissions.",
        ServiceWorkerError::WorkerRegistration(_) => "Error registering service worker.",
        ServiceWorkerError::RequestPermission(_) => "Error requesting notification permissions.",
        ServiceWorkerError::SerdeJson(_) => "Serde failed.",
        ServiceWorkerError::WorkerUnregistration(_) => "Failed to unregister service worker.",
    };
    error!(message, worker_error);
    format!("{}: {:?}", message, worker_error)
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
            attrs!{At::Disabled => model.worker_data.is_none().as_at_value()},
            "Request Push Subscription",
            ev(Ev::Click, |_| Msg::RequestNotificationPermission),
        ],
        button![
            attrs!{At::Disabled => {
                match &model.worker_data {
                    Some(worker_data) if worker_data.notifications_granted => false,
                    _ => true,
                }.as_at_value()
            }},
            "Send Message",
            ev(Ev::Click, |_| Msg::SendMessage),
        ],
        button![
            "Clear Cache",
            ev(Ev::Click, |_| Msg::ClearCache),
        ],
        button![
            attrs!{At::Disabled => model.worker_data.is_none().as_at_value()},
            "Unregister Service Worker",
            ev(Ev::Click, |_| Msg::UnregisterWorker),
        ],
        model.error.as_ref().map(|error| {
            p![
                style!{St::Color => "red"},
                error
            ]
        }),
        model.message.as_ref().map(|message| {
            p![
                message
            ]
        }),
        h2!["Push Subscription"],
        model.worker_data.as_ref().map(|worker_data| {
            code![serde_json::to_string_pretty(&worker_data.push_subscription).expect("stringify `push_subscription`")]
        }),
        br![],
        br![],
        img![attrs! {
            At::Src => "/public/images/important-notes.png"
        },],
        br![],
        a![
            attrs! {
                At::Href => "https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API/Using_Service_Workers"
            },
            "Using Service Workers (Service Worker Api)"
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
