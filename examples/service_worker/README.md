# Service Worker example

Service worker is an exciting technology that is available in most major browsers. To summarize, it can be used to cache
assets, providing a positive offline experience. Additionally, service worker has the ability to send notifications that
are generated both locally from a web application as well as from a remote server.

The example in this crate demonstrates the following features:

1. Use service worker to cache all assets (including the generated wasm file).
1. Register the service worker.
1. If the service worker is not yet activated, an even listener will be registered, waiting for the
   state to reach "activated".
1. When the state reaches "activated", the Notification object will request permission for notifications.
1. If permission is granted, the PushManager will subscribe to the service using an example vapid key.
1. Finally, a PushSubscription will be returned, containing the information that can be passed to a
   notification back-end server.

---

## Running

```bash
cargo make start
```

- Open [http://127.0.0.1:8000/](http://127.0.0.1:8000/) in your browser. This will cache the assets.
- Kill the running cargo process to terminate the local dev server.
- Refresh the browser and notice that the page loads with all assets.
- Click on the `Send Message` button. A notification should pop up on the browser.
