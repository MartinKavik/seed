importScripts('https://storage.googleapis.com/workbox-cdn/releases/5.1.2/workbox-sw.js');

// workbox.core - Provides core workbox functionality. Ths will be used for service worker updating.
const core = workbox.core;

// workbox.precaching - Helps to simplify the caching process
const precaching = workbox.precaching;

// We want to publish a new service worker and immediately update and control the page.
// - https://developers.google.com/web/tools/workbox/modules/workbox-core#skip_waiting_and_clients_claim
core.skipWaiting();
core.clientsClaim();

// Cache all of the assets for offline viewing. This can be done manually or by using a tool, such as 
// `workbox-cli`- https://developers.google.com/web/tools/workbox/modules/workbox-cli.
// By updating the revision hash after an asset has been updated, the cached resource will be
// updated in the browser's cache.
precaching.precacheAndRoute(
  [
    { "revision": "12345", "url": "custom.js" },
    // Wasm bindgen puts any module js snippets into the following directory: pkg/snippets/<crate_name>-<hash>/
    // The purpose of the hash is to support snippets and different versions of the crate. For example, this
    // hash will change if you change the version in the Cargo.toml file. If the version is updated, this entry
    // will need to be updated as well. Alternatively, builders, such as webpack or workbox-cli can generate this
    // for you in the build process. Note that this will go away once https://github.com/rustwasm/wasm-bindgen/pull/2288 
    // makes its way into the next wasm_bindgen release.
    { "revision": "12345", "url": "pkg/snippets/service_worker-a3e76e61c3718edf/src/subscribe.js" },
    { "revision": "12345", "url": "index.html" },
    { "revision": "12345", "url": "/" },
    { "revision": "12345", "url": "pkg/package_bg.wasm" }, 
    { "revision": "12345", "url": "pkg/package.js" },
    { "revision": "12345", "url": "images/important-notes.png" }
  ]
);

// Listen for and display a default push notification if the push event is triggered.
self.addEventListener('push', (event) => {
  const title = 'Seed service worker!';
  const options = {
    body: event.data.text()
  };
  event.waitUntil(self.registration.showNotification(title, options));
});
