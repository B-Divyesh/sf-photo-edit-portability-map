const PRODUCT = "photo-edit-portability-map";
const API = `https://api.sociobot.in/api/v1/products/${PRODUCT}`;
const TOKEN_KEY = `sb_license:${PRODUCT}`;
const VERDICT_KEY = `${TOKEN_KEY}:verdict`;
const DAY = 86_400_000;

const demoRows = [
  { state: "E", tone: "embedded", field: "Camera and lens EXIF", records: "18,420", support: { default: "supported" }, action: "Confirm camera, lens, ISO, and orientation on three files." },
  { state: "S", tone: "sidecar", field: "Corrected capture date", records: "614", support: { default: "supported", generic: "verify" }, action: "Check a corrected video date after the destination rescan." },
  { state: "S", tone: "sidecar", field: "Star rating", records: "7,382", support: { immich: "partial", default: "supported", generic: "verify" }, action: "Confirm 0, 3, and 5-star examples." },
  { state: "S", tone: "sidecar", field: "Keywords and hierarchy", records: "12,110", support: { immich: "partial", default: "supported", generic: "verify" }, action: "Check one nested keyword branch; hierarchy may flatten." },
  { state: "C", tone: "catalog", field: "Collections", records: "86", support: { digikam: "partial", default: "unsupported" }, action: "Export collection membership or recreate it as tags." },
  { state: "C", tone: "catalog", field: "Virtual copies", records: "223", support: { default: "unsupported" }, action: "Render or duplicate every version you intend to keep." },
  { state: "!", tone: "unsupported", field: "Lightroom develop recipe", records: "15,906", support: { default: "unsupported" }, action: "Render critical finals; preserve RAW files and the catalog." }
];

const supportLabel = { supported: "Supported", partial: "Partial — verify", unsupported: "Unsupported", verify: "Unknown — verify" };
const profileNames = { immich: "Immich", darktable: "darktable", digikam: "digiKam", generic: "a generic folder" };

function renderDemo(profile) {
  const body = document.querySelector("#report-body");
  body.replaceChildren(...demoRows.map((row) => {
    const result = row.support[profile] ?? row.support.default;
    const tr = document.createElement("tr");
    const cells = [
      `<i class="state ${row.tone}">${row.state}</i>`,
      row.field,
      row.records,
      `<span class="support ${result}">${supportLabel[result]}</span>`,
      row.action
    ];
    ["State", "Field", "Records", "Target result", "Before you move"].forEach((label, index) => {
      const td = document.createElement("td");
      td.dataset.label = label;
      if (index === 4) td.className = "action";
      td.innerHTML = cells[index];
      tr.append(td);
    });
    return tr;
  }));
  document.querySelector("#profile-note").textContent = `Showing the conservative ${profileNames[profile]} capability profile. Your installed version is the final authority.`;
}

async function copyText(value, button) {
  try {
    await navigator.clipboard.writeText(value);
  } catch {
    const area = document.createElement("textarea");
    area.value = value;
    area.style.position = "fixed";
    area.style.opacity = "0";
    document.body.append(area);
    area.select();
    document.execCommand("copy");
    area.remove();
  }
  const original = button.textContent;
  button.textContent = "Copied";
  showToast("Command copied. Your files stay local.");
  window.setTimeout(() => { button.textContent = original; }, 1600);
}

let toastTimer;
function showToast(message) {
  const toast = document.querySelector("#toast");
  toast.textContent = message;
  toast.classList.add("show");
  clearTimeout(toastTimer);
  toastTimer = window.setTimeout(() => toast.classList.remove("show"), 2400);
}

function setLicenseStatus(message, state = "") {
  const status = document.querySelector("#license-status");
  status.textContent = message;
  status.className = `license-status ${state}`.trim();
}

function revealCliActivation(token) {
  const button = document.querySelector("#copy-license");
  if (!/^[A-Za-z0-9._~-]{8,4096}$/.test(token)) {
    button.hidden = true;
    return false;
  }
  button.dataset.token = token;
  button.hidden = false;
  return true;
}

function cachedVerdict() {
  try { return JSON.parse(localStorage.getItem(VERDICT_KEY)); } catch { return null; }
}

async function verifyLicense(token, force = false) {
  const cached = cachedVerdict();
  if (!force && cached?.valid && Date.now() - cached.checkedAt < DAY) {
    setLicenseStatus("Pro active. License was verified within the last day.", "active");
    return;
  }
  if (cached?.valid) setLicenseStatus("Pro active. Rechecking quietly…", "active");
  else setLicenseStatus("Checking this license…");
  try {
    const response = await fetch(`${API}/verify?license=${encodeURIComponent(token)}`, { headers: { accept: "application/json" } });
    if (!response.ok) throw new Error("verification service unavailable");
    const result = await response.json();
    localStorage.setItem(VERDICT_KEY, JSON.stringify({ valid: result.valid === true, checkedAt: Date.now() }));
    if (result.valid === true) {
      setLicenseStatus("Pro active. Verification samples up to 100 files are unlocked.", "active");
      showToast("License verified. Pro is active.");
    } else {
      setLicenseStatus("This license is no longer active. Check the token or purchase a new license.", "error");
    }
  } catch {
    if (cached?.valid) setLicenseStatus("Pro active from the last verified check. Offline recheck postponed.", "active");
    else setLicenseStatus("License verification is unavailable. Your free tools still work; reconnect and try again.", "error");
  }
}

function acceptReturnLicense() {
  const url = new URL(window.location.href);
  const token = url.searchParams.get("license")?.trim();
  if (!token) {
    const stored = localStorage.getItem(TOKEN_KEY);
    if (stored) revealCliActivation(stored);
    return stored;
  }
  if (!revealCliActivation(token)) {
    setLicenseStatus("The returned license has an unexpected format. Paste it below to retry.", "error");
    return null;
  }
  localStorage.setItem(TOKEN_KEY, token);
  url.searchParams.delete("license");
  history.replaceState({}, "", `${url.pathname}${url.search}${url.hash}`);
  return token;
}

function updateConnection() {
  const node = document.querySelector("#connection");
  const offline = !navigator.onLine;
  node.classList.toggle("offline", offline);
  node.lastChild.textContent = offline ? "Offline · local demo ready" : "Local-first";
}

document.querySelector("#profile").addEventListener("change", (event) => renderDemo(event.target.value));
document.querySelectorAll("[data-copy]").forEach((button) => button.addEventListener("click", () => copyText(button.dataset.copy, button)));
document.querySelector("#license-form").addEventListener("submit", (event) => {
  event.preventDefault();
  const token = new FormData(event.currentTarget).get("license").trim();
  if (!token || !revealCliActivation(token)) {
    setLicenseStatus("That license format is not recognized. Check the complete token and try again.", "error");
    return;
  }
  localStorage.setItem(TOKEN_KEY, token);
  event.currentTarget.reset();
  verifyLicense(token, true);
});
document.querySelector("#copy-license").addEventListener("click", (event) => {
  copyText(`edit-portability-map license activate ${event.currentTarget.dataset.token}`, event.currentTarget);
});
window.addEventListener("online", updateConnection);
window.addEventListener("offline", updateConnection);

renderDemo("immich");
updateConnection();
const license = acceptReturnLicense();
if (license) {
  const cached = cachedVerdict();
  if (cached?.valid) setLicenseStatus("Pro active from your last verified check.", "active");
  verifyLicense(license);
}
if ("serviceWorker" in navigator && location.protocol === "https:") navigator.serviceWorker.register("/sw.js");
