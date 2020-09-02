use seed::prelude::*;

#[derive(Debug)]
pub enum ServiceWorkerError {
    DeletingCacheKeys(JsValue),
    GetCaches(JsValue),
    GetCacheKeys(JsValue),
    GetServiceWorkerRegistration(JsValue),
    GetWindow,
    InvalidState,
    InvalidPermissions,
    Registration(JsValue),
    RequestPermission(JsValue),
    RetrievePushManager(JsValue),
    SerdeJson(serde_json::error::Error),
    StateChangeListener(JsValue),
    UnregisterServiceWorker(JsValue),
}

impl std::fmt::Display for ServiceWorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DeletingCacheKeys(_) => write!(f, "Failed to delete cache keys."),
            Self::GetCaches(_) => write!(f, "Couldn't get caches."),
            Self::GetCacheKeys(_) => write!(f, "Couldn't get cache keys."),
            Self::GetServiceWorkerRegistration(_) => {
                write!(f, "Couldn't get service worker registration.")
            }
            Self::GetWindow => write!(f, "Couldn't retrieve window."),
            Self::InvalidState => write!(
                f,
                "Service worker is not in the installing, waiting, or active state."
            ),
            Self::InvalidPermissions => write!(f, "User has not granted notification permissions."),
            Self::Registration(_) => write!(f, "Error registering service worker."),
            Self::RequestPermission(_) => write!(f, "Error requesting notification permissions."),
            Self::RetrievePushManager(_) => write!(f, "Error retrieving push manager."),
            Self::SerdeJson(err) => write!(f, "{}", err),
            Self::StateChangeListener(_) => write!(
                f,
                "Error encountered while listening for state changes on the service worker."
            ),
            Self::UnregisterServiceWorker(_) => write!(f, "Failed to unregister service worker."),
        }
    }
}

impl From<serde_json::error::Error> for ServiceWorkerError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::SerdeJson(err)
    }
}
