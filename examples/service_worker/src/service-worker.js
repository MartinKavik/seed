importScripts('https://storage.googleapis.com/workbox-cdn/releases/5.1.2/workbox-sw.js');

const strategies = workbox.strategies;
const core = workbox.core;
const routing = workbox.routing;
const precaching = workbox.precaching;

core.skipWaiting();
core.clientsClaim();

precaching.precacheAndRoute(self.__WB_MANIFEST);

self.addEventListener('push', (event) => {
  const title = 'Seed service worker!';
  const options = {
    body: event.data.text()
  };
  event.waitUntil(self.registration.showNotification(title, options));
});
