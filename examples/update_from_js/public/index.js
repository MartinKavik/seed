import init from '/pkg/package.js';
import { start, a_rust_function } from '/pkg/package.js';

window.enableClock = () => {
  const sendTick = () => {
    tick(new Date().toLocaleTimeString());
  };
  sendTick();

  setInterval(() => {
    sendTick();
  }, 1000);
};

init('/pkg/package_bg.wasm').then(() => {
  const [js_ready, tick] = start();
  window.tick = tick;
  js_ready(true);

  a_rust_function(1, 2, 3);
});

