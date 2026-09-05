const PRODUCT = "photo-edit-portability-map";
const API = `https://api.sociobot.in/api/v1/products/${PRODUCT}`;
const TOKEN_KEY = `sb_license:${PRODUCT}`;
const VERDICT_KEY = `${TOKEN_KEY}:verdict`;
const DAY = 86_400_000;
const DEMO_KEY = "demo:photo-edit-portability-map:state";

const demoRows = [
  { state: "E", tone: "embedded", field: "Camera and lens EXIF", records: "3", support: { default: "supported" }, action: "Confirm camera, lens, ISO, and orientation." },
  { state: "S", tone: "sidecar", field: "Keywords", records: "2", support: { generic: "verify", default: "supported" }, action: "Check one keyword after import." },
  { state: "C", tone: "catalog", field: "Corrected capture date", records: "2", support: { generic: "verify", default: "supported" }, action: "Check the corrected date after import." },
  { state: "C", tone: "catalog", field: "Star rating", records: "1", support: { immich: "partial", generic: "verify", default: "supported" }, action: "Confirm a 3-star example." },
  { state: "!", tone: "unsupported", field: "Lightroom develop recipe", records: "3", support: { default: "unsupported" }, action: "Render finished versions that you need." }
];

const supportLabel = { supported: "Supported", partial: "Partial — check", unsupported: "Unsupported", verify: "Check" };
const profileNames = { immich: "Immich", darktable: "darktable", digikam: "digiKam", generic: "generic folder" };

function readDemoState() {
  try {
    const parsed = JSON.parse(localStorage.getItem(DEMO_KEY));
    return profileNames[parsed?.profile] ? parsed : { profile: "immich" };
  } catch {
    return { profile: "immich" };
  }
}

function writeDemoState(state) {
  localStorage.setItem(DEMO_KEY, JSON.stringify(state));
}

function showToast(message) {
  const toast = document.querySelector("#toast");
  if (!toast) return;
  toast.textContent = message;
  toast.classList.add("show");
  window.clearTimeout(showToast.timer);
  showToast.timer = window.setTimeout(() => toast.classList.remove("show"), 2400);
}

async function copyText(value, button) {
  try {
    await navigator.clipboard.writeText(value);
  } catch {
    const area = document.createElement("textarea");
    area.value = value;
    area.className = "clipboard-fallback";
    document.body.append(area);
    area.select();
    document.execCommand("copy");
    area.remove();
  }
  const original = button.textContent;
  button.textContent = "Copied";
  showToast("Command copied.");
  window.setTimeout(() => { button.textContent = original; }, 1600);
}

function updateConnection() {
  const node = document.querySelector("#connection");
  if (!node) return;
  const offline = !navigator.onLine;
  node.classList.toggle("offline", offline);
  node.lastChild.textContent = offline ? "Offline" : document.body.dataset.page === "demo" ? "Sample only" : "Local CLI";
}

function setLicenseStatus(message, state = "") {
  const status = document.querySelector("#license-status");
  if (!status) return;
  status.textContent = message;
  status.className = `license-status ${state}`.trim();
}

function revealLicenseCommand(token) {
  const button = document.querySelector("#copy-license");
  if (!button || !/^[A-Za-z0-9._~-]{8,4096}$/.test(token)) return false;
  button.dataset.token = token;
  button.hidden = false;
  return true;
}

function cachedVerdict() {
  try { return JSON.parse(localStorage.getItem(VERDICT_KEY)); } catch { return null; }
}

async function verifyLicense(token, force = false) {
  const cached = cachedVerdict();
  const matching = cached?.token === token ? cached : null;
  if (!force && matching?.valid && Date.now() - matching.checkedAt < DAY) {
    setLicenseStatus("Pro is active from a recent verification.", "active");
    return;
  }
  setLicenseStatus(matching?.valid ? "Pro is active. Checking again…" : "Checking this license…", matching?.valid ? "active" : "");
  try {
    const response = await fetch(`${API}/verify?license=${encodeURIComponent(token)}`, { headers: { accept: "application/json" } });
    if (!response.ok) throw new Error("verification service unavailable");
    const result = await response.json();
    localStorage.setItem(VERDICT_KEY, JSON.stringify({ token, valid: result.valid === true, checkedAt: Date.now() }));
    if (result.valid === true) {
      setLicenseStatus("Pro is active. Larger verification samples are available.", "active");
      showToast("License verified.");
    } else {
      setLicenseStatus("This license is not active. Check the token or buy Pro.", "error");
    }
  } catch {
    setLicenseStatus(matching?.valid ? "Pro stays active from its last verification. Reconnect to check again." : "License verification is unavailable. Your free scan still works.", matching?.valid ? "active" : "error");
  }
}

function initialiseLicense() {
  const form = document.querySelector("#license-form");
  if (!form) return;
  const url = new URL(window.location.href);
  const returned = url.searchParams.get("license")?.trim();
  const token = returned || localStorage.getItem(TOKEN_KEY);
  if (returned) {
    if (revealLicenseCommand(returned)) {
      localStorage.setItem(TOKEN_KEY, returned);
      url.searchParams.delete("license");
      history.replaceState({}, "", `${url.pathname}${url.search}${url.hash}`);
    } else {
      setLicenseStatus("The returned license format is not recognized. Paste it below to retry.", "error");
    }
  }
  if (token && revealLicenseCommand(token)) verifyLicense(token);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    const next = new FormData(form).get("license").trim();
    if (!revealLicenseCommand(next)) {
      setLicenseStatus("Paste the complete license token and try again.", "error");
      return;
    }
    localStorage.setItem(TOKEN_KEY, next);
    form.reset();
    verifyLicense(next, true);
  });
  document.querySelector("#copy-license").addEventListener("click", (event) => copyText(`edit-portability-map license activate ${event.currentTarget.dataset.token}`, event.currentTarget));
}

function renderDemo(profile) {
  const body = document.querySelector("#demo-report-body");
  if (!body) return;
  body.replaceChildren(...demoRows.map((row) => {
    const result = row.support[profile] ?? row.support.default;
    const tr = document.createElement("tr");
    const values = [
      { label: "State", type: "state" },
      { label: "Field", text: row.field },
      { label: "Records", text: row.records },
      { label: "Target result", type: "support" },
      { label: "What to do", text: row.action, action: true }
    ];
    for (const value of values) {
      const cell = document.createElement("td");
      cell.dataset.label = value.label;
      if (value.action) cell.className = "action";
      if (value.type === "state") {
        const state = document.createElement("i");
        state.className = `state ${row.tone}`;
        state.textContent = row.state;
        cell.append(state, ` ${row.tone === "catalog" ? "Catalog-only" : row.tone === "sidecar" ? "Sidecar" : row.tone === "embedded" ? "Embedded" : "Unsupported"}`);
      } else if (value.type === "support") {
        const support = document.createElement("span");
        support.className = `support ${result}`;
        support.textContent = supportLabel[result];
        cell.append(support);
      } else {
        cell.textContent = value.text;
      }
      tr.append(cell);
    }
    return tr;
  }));
  document.querySelector("#profile-note").textContent = `Showing the ${profileNames[profile]} capability profile. Check your installed version before migration.`;
}

function initialiseDemo() {
  const state = readDemoState();
  writeDemoState(state);
  const profile = document.querySelector("#profile");
  profile.value = state.profile;
  renderDemo(state.profile);
  profile.addEventListener("change", () => {
    const next = { profile: profile.value };
    writeDemoState(next);
    renderDemo(next.profile);
  });
  document.querySelector("#reset-demo").addEventListener("click", () => {
    const reset = { profile: "immich" };
    writeDemoState(reset);
    profile.value = reset.profile;
    renderDemo(reset.profile);
    showToast("Demo reset. Sample data is unchanged.");
  });
  document.querySelector("#start-real").addEventListener("click", () => localStorage.removeItem(DEMO_KEY));
}

function leaveDemoIfRequested() {
  const url = new URL(window.location.href);
  if (url.searchParams.get("start") !== "real") return;
  localStorage.removeItem(DEMO_KEY);
  url.searchParams.delete("start");
  history.replaceState({}, "", `${url.pathname}${url.search}${url.hash}`);
}

document.querySelectorAll("[data-copy]").forEach((button) => button.addEventListener("click", () => copyText(button.dataset.copy, button)));
window.addEventListener("online", updateConnection);
window.addEventListener("offline", updateConnection);
leaveDemoIfRequested();
if (document.body.dataset.page === "demo") initialiseDemo();
initialiseLicense();
updateConnection();
if ("serviceWorker" in navigator && location.protocol === "https:") navigator.serviceWorker.register("/sw.js");
