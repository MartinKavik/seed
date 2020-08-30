use seed::prelude::*;

#[derive(Debug)]
pub enum ServiceWorkerError {
    GetWindow,
    InvalidState,
    InvalidPermissions,
    MapAToB(JsValue),
    Registration(JsValue),
    RequestPermission(JsValue),
    RetrievePushManager(JsValue),
    SerdeJson(serde_json::error::Error),
    StateChangeListener(JsValue),
}

impl std::fmt::Display for ServiceWorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ServiceWorkerError::GetWindow => write!(f, "Couldn't retrieve window."),
            ServiceWorkerError::InvalidState => write!(
                f,
                "Service worker is not in the installing, waiting, or active state."
            ),
            ServiceWorkerError::InvalidPermissions => {
                write!(f, "User has not granted notification permissions.")
            }
            ServiceWorkerError::MapAToB(_) => write!(f, "Error encoding the base64 string."),
            ServiceWorkerError::Registration(_) => write!(f, "Error registering service worker."),
            ServiceWorkerError::RequestPermission(_) => {
                write!(f, "Error requesting notification permissions.")
            }
            ServiceWorkerError::RetrievePushManager(_) => {
                write!(f, "Error retrieving push manager.")
            }
            ServiceWorkerError::SerdeJson(ref err) => write!(f, "{}", err),
            ServiceWorkerError::StateChangeListener(_) => write!(
                f,
                "Error encountered while listening for state changes on the service worker."
            ),
        }
    }
}

impl From<serde_json::error::Error> for ServiceWorkerError {
    fn from(err: serde_json::error::Error) -> Self {
        ServiceWorkerError::SerdeJson(err)
    }
}
