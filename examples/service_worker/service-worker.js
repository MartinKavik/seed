importScripts('https://storage.googleapis.com/workbox-cdn/releases/5.1.2/workbox-sw.js');

const strategies = workbox.strategies;
const core = workbox.core;
const routing = workbox.routing;
const precaching = workbox.precaching;

core.skipWaiting();
core.clientsClaim();

precaching.precacheAndRoute([{"revision":"caf4ec0a00de3a356dc4a9fdd7d1d54f","url":"custom.js"},{"revision":"c5d9009f1ee65b5ecc254670ecd29cab","url":"index.html"},{"revision":"7eca533795667021f9f1c716cb6c287a","url":"package.json"},{"revision":"22d56a61a849f8523fdf931b767846d8","url":"pkg/package_bg.d.ts"},{"revision":"71f812e5324b73946b47a1531cc8a362","url":"pkg/package_bg.wasm"},{"revision":"a0bb58ef861330b23d4a6c8b4a0d38ed","url":"pkg/package.d.ts"},{"revision":"81e1fc31945f1002b26b48fa2a0c207a","url":"pkg/package.js"},{"revision":"ffbdde289cf7242c750be75f9dcd43be","url":"pkg/package.json"},{"revision":"caf4ec0a00de3a356dc4a9fdd7d1d54f","url":"pkg/snippets/service_worker-a3e76e61c3718edf/custom.js"},{"revision":"92f90543b0b2592b97668168be25b0a4","url":"workbox-config.js"}]);

self.addEventListener('push', (event) => {
  const title = 'Seed service worker!';
  const options = {
    body: event.data.text()
  };
  event.waitUntil(self.registration.showNotification(title, options));
});
