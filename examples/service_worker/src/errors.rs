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
            Self::GetWindow => write!(f, "Couldn't retrieve window."),
            Self::InvalidState => write!(
                f,
                "Service worker is not in the installing, waiting, or active state."
            ),
            Self::InvalidPermissions => write!(f, "User has not granted notification permissions."),
            Self::MapAToB(_) => write!(f, "Error encoding the base64 string."),
            Self::Registration(_) => write!(f, "Error registering service worker."),
            Self::RequestPermission(_) => write!(f, "Error requesting notification permissions."),
            Self::RetrievePushManager(_) => write!(f, "Error retrieving push manager."),
            Self::SerdeJson(ref err) => write!(f, "{}", err),
            Self::StateChangeListener(_) => write!(
                f,
                "Error encountered while listening for state changes on the service worker."
            ),
        }
    }
}

impl From<serde_json::error::Error> for ServiceWorkerError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::SerdeJson(err)
    }
}
