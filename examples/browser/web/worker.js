// Workers Static Assets caps a file at 25 MiB, so the browser's wasm lives
// in R2. Its name carries a content hash, so it never changes once uploaded.
export default {
  async fetch(request, env) {
    const name = new URL(request.url).pathname.slice(1);
    const object = await env.WASM.get(name, { onlyIf: request.headers });
    if (object === null) return env.ASSETS.fetch(request);
    const headers = new Headers({
      "content-type": "application/wasm",
      "cache-control": "public, max-age=31536000, immutable",
      etag: object.httpEtag,
    });
    return "body" in object
      ? new Response(object.body, { headers })
      : new Response(null, { status: 304, headers });
  },
};
