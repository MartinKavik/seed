importScripts('https://storage.googleapis.com/workbox-cdn/releases/5.1.2/workbox-sw.js');

const strategies = workbox.strategies;
const core = workbox.core;
const routing = workbox.routing;
const precaching = workbox.precaching;

core.skipWaiting();
core.clientsClaim();

precaching.precacheAndRoute([{"revision":"caf4ec0a00de3a356dc4a9fdd7d1d54f","url":"custom.js"},{"revision":"c5d9009f1ee65b5ecc254670ecd29cab","url":"index.html"},{"revision":"7eca533795667021f9f1c716cb6c287a","url":"package.json"},{"revision":"37a363f559096750e8e8e4906a144d67","url":"pkg/package_bg.d.ts"},{"revision":"b0956963b91185b4c200558d8101a8df","url":"pkg/package_bg.wasm"},{"revision":"584a2a5ad4acc26fce845ad3ab50d850","url":"pkg/package.d.ts"},{"revision":"0bdef4129c2d146a4baf18eb80dcb06c","url":"pkg/package.js"},{"revision":"ffbdde289cf7242c750be75f9dcd43be","url":"pkg/package.json"},{"revision":"caf4ec0a00de3a356dc4a9fdd7d1d54f","url":"pkg/snippets/service_worker-a3e76e61c3718edf/custom.js"},{"revision":"92f90543b0b2592b97668168be25b0a4","url":"workbox-config.js"}]);

self.addEventListener('push', (event) => {
  const title = 'Seed service worker!';
  const options = {
    body: event.data.text()
  };
  event.waitUntil(self.registration.showNotification(title, options));
});
