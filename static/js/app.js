console.log("Solid SSG CMS frontend loaded");

async function api(path, options = {}) {
  const res = await fetch(`/api${path}`, {
    headers: { "Content-Type": "application/json", ...options.headers },
    ...options,
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(err.error || res.statusText);
  }
  if (res.status === 204) return null;
  return res.json();
}

window.SolidCMS = {
  list: () => api("/content"),
  get: (slug) => api(`/content/by-slug/${slug}`),
  create: (data) => api("/content", { method: "POST", body: JSON.stringify(data) }),
  publish: (id) => api(`/content/by-id/${id}/publish`, { method: "POST" }),
  build: () => api("/build", { method: "POST" }),
  health: () => api("/health"),
};
