use seed::prelude::*;

#[derive(Debug)]
pub enum ServiceWorkerError {
    GetWindowError,
    InvalidStateError,
    InvalidPermissionsError,
    MapAToBError(JsValue),
    RegistrationError(JsValue),
    RequestPermissionError(JsValue),
    RetrievePushManagerError(JsValue),
    SerdeJsonError(serde_json::error::Error),
    StateChangeListenerError(JsValue),
}

impl std::fmt::Display for ServiceWorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ServiceWorkerError::GetWindowError => write!(f, "Couldn't retrieve window."),
            ServiceWorkerError::InvalidStateError => write!(
                f,
                "Service worker is not in the installing, waiting, or active state."
            ),
            ServiceWorkerError::InvalidPermissionsError => {
                write!(f, "User has not granted notification permissions.")
            }
            ServiceWorkerError::MapAToBError(_) => write!(f, "Error encoding the base64 string."),
            ServiceWorkerError::RegistrationError(_) => {
                write!(f, "Error registering service worker.")
            }
            ServiceWorkerError::RequestPermissionError(_) => {
                write!(f, "Error requesting notification permissions.")
            }
            ServiceWorkerError::RetrievePushManagerError(_) => {
                write!(f, "Error retrieving push manager.")
            }
            ServiceWorkerError::SerdeJsonError(ref err) => write!(f, "{}", err),
            ServiceWorkerError::StateChangeListenerError(_) => write!(
                f,
                "Error encountered while listening for state changes on the service worker."
            ),
        }
    }
}

impl From<serde_json::error::Error> for ServiceWorkerError {
    fn from(err: serde_json::error::Error) -> Self {
        ServiceWorkerError::SerdeJsonError(err)
    }
}
