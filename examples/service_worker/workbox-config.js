module.exports = {
  "globDirectory": ".",
  "globPatterns": [
    "**/*.{js,html,wasm}"
  ],
  "swDest": "./service-worker.js",
  "swSrc": "src/service-worker.js",
  "maximumFileSizeToCacheInBytes": 100000000,
};