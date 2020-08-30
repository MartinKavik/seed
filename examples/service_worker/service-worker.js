importScripts('https://storage.googleapis.com/workbox-cdn/releases/5.1.2/workbox-sw.js');

const strategies = workbox.strategies;
const core = workbox.core;
const routing = workbox.routing;
const precaching = workbox.precaching;

core.skipWaiting();
core.clientsClaim();

precaching.precacheAndRoute(
  [
    { "revision": "caf4ec0a00de3a356dc4a9fdd7d1d54f", "url": "custom.js" }, 
    { "revision": "caf4ec0a00de3a356dc4a9fdd7d1d54f", "url": "pkg/snippets/service_worker-a3e76e61c3718edf/custom.js" },
    { "revision": "c5d9009f1ee65b5ecc254670ecd29cab", "url": "index.html" }, 
    { "revision": "94297a6d400252a3afc46260226069eb", "url": "pkg/package_bg.wasm" }, 
    { "revision": "29e75c315196d901591f613e2975ccfd", "url": "pkg/package.js" },
    { "revision": "caf4ec0a00de3a356dc4a9fdd7d1d54f", "url": "images/important-notes.png" }
  ]
);

self.addEventListener('push', (event) => {
  const title = 'Seed service worker!';
  const options = {
    body: event.data.text()
  };
  event.waitUntil(self.registration.showNotification(title, options));
});
