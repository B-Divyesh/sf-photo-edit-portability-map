const CACHE = "edit-portability-map-v5";
const SHELL = ["/", "/demo/", "/privacy/", "/terms/", "/404.html", "/portability-landscape-720.webp", "/portability-landscape.webp", "/portability-map-card.webp", "/apple-touch-icon.png", "/demo-terminal.svg", "/favicon.svg", "/manifest.webmanifest"];

self.addEventListener("install", (event) => event.waitUntil(caches.open(CACHE).then((cache) => cache.addAll(SHELL))));
self.addEventListener("activate", (event) => event.waitUntil(caches.keys().then((keys) => Promise.all(keys.filter((key) => key !== CACHE).map((key) => caches.delete(key))))));
self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET" || new URL(event.request.url).origin !== location.origin) return;
  event.respondWith(fetch(event.request).then((response) => {
    const copy = response.clone();
    caches.open(CACHE).then((cache) => cache.put(event.request, copy));
    return response;
  }).catch(() => caches.match(event.request).then((cached) => cached || caches.match("/"))));
});
