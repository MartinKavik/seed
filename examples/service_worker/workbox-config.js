module.exports = {
  "globDirectory": ".",
  "globPatterns": [
    "**/*.{js,html,ts,wasm,json}"
  ],
  "swDest": "./service-worker.js",
  "swSrc": "src/service-worker.js",
  "maximumFileSizeToCacheInBytes": 100000000,
};