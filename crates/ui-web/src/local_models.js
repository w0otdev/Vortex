let dispose = () => {};

const COPY = {
vi: {
title: "Mô hình chạy trên máy",
note: "Mô hình chạy trực tiếp trên thiết bị để phân loại và tóm tắt thư. Dung lượng bên dưới là kích thước tệp tải về, khi chạy sẽ dùng thêm RAM.",
loading: "Đang kiểm tra Ollama...",
download: "Tải mô hình",
use: "Dùng mô hình",
cancel: "Hủy tải",
refresh: "Kiểm tra lại",
ready: "Đã tải",
selected: "Đã chọn mô hình. Bấm Lưu cài đặt để áp dụng.",
missing: "Hãy cài đặt hoặc bật Ollama rồi bấm Kiểm tra lại.",
install: "Tải Ollama",
source: " Nguồn mô hình",
downloading: "Đang tải",
file: "Tiến độ tệp",
canceling: "Đang hủy tải...",
canceled: "Đã hủy tải.",
failed: "Không tải được mô hình.",
other: "Đang tải mô hình khác:",
timeout: "Đã tạm dừng theo dõi tiến độ. Mở lại cửa sổ này để xem tiếp.",
},
en: {
title: "Local models",
note: "Runs on your device to classify and summarize mail. Listed sizes are download sizes; running the model uses additional RAM.",
loading: "Checking Ollama...",
download: "Download model",
use: "Use model",
cancel: "Cancel download",
refresh: "Check again",
ready: "Downloaded",
selected: "Model selected. Save settings to apply.",
missing: "Install or start Ollama, then check again.",
install: "Download Ollama",
source: "Model source",
downloading: "Downloading",
file: "File progress",
canceling: "Canceling download...",
canceled: "Download canceled.",
failed: "Could not download model.",
other: "Downloading another model:",
timeout: "Paused progress polling. Reopen this window to check status.",
},
};

const MSG_VI = {
"Installing Ollama...": "Đang cài đặt Ollama...",
"Downloading Ollama installer...": "Đang tải bộ cài Ollama...",
"Starting Ollama...": "Đang khởi động Ollama...",
"Pulling model...": "Đang tải tệp mô hình...",
};

const endpointValue = (input) => input.value.trim().replace(/\/+$/, "");
const sizeLabel = (bytes) => `~${Math.round(bytes / 1_000_000)} MB`;
const inProgress = (job) => job && ["downloading", "canceling"].includes(job.status);

export function open(api) {
dispose();
const $ = (id) => document.getElementById(id);
const dialog = $("aiDialog"), host = $("aiLocalModels");
const endpointInput = $("aiEndpoint"), modelInput = $("aiModel");
if (!dialog.open || $("aiProvider").value !== "ollama") return;
const endpoint = endpointValue(endpointInput);
const copy = COPY[document.documentElement.lang] || COPY.en;
let active = true, timer, catalog, job, downloading = false, errorMessage = "";
let catalogRequest = 0, statusRequest = 0;
const started = Date.now();
const current = () => active && dialog.open && $("aiProvider").value === "ollama"
&& endpointValue(endpointInput) === endpoint;
const ownJob = () => job && job.endpoint === endpoint;
const stop = () => {
active = false;
clearTimeout(timer);
dialog.removeEventListener("close", stop);
};
dispose = stop;
dialog.addEventListener("close", stop);

function node(tag, text, className) {
const element = document.createElement(tag);
if (text) element.textContent = text;
if (className) element.className = className;
return element;
}

const label = node("label", copy.title, "field-label");
label.htmlFor = "localModelChoice";
const select = node("select", "", "input-field");
select.id = "localModelChoice";
const note = node("p", copy.note, "field-hint");
const source = node("a", copy.source, "field-hint");
source.target = "_blank"; source.rel = "noopener noreferrer";
source.style.marginRight = "14px";
const install = node("a", copy.install, "field-hint");
install.href = "https://ollama.com/download";
install.target = "_blank"; install.rel = "noopener noreferrer";
const actions = node("div", "", "reader-actions");
const download = node("button", copy.download, "btn btn-sm");
const cancel = node("button", copy.cancel, "btn btn-sm");
const refresh = node("button", copy.refresh, "btn btn-sm");
for (const button of [download, cancel, refresh]) button.type = "button";
const progress = node("progress");
progress.setAttribute("aria-label", copy.file);
const status = node("p", copy.loading, "field-hint");
status.setAttribute("role", "status");
actions.append(download, cancel, refresh);
host.replaceChildren(label, select, note, source, install, actions, progress, status);

function render() {
const model = catalog?.models.find((entry) => entry.id === select.value);
const busy = inProgress(job);
download.disabled = downloading || !model || (!model.installed && busy);
download.textContent = model?.installed ? copy.use : copy.download;
select.disabled = downloading;
cancel.hidden = !busy;
cancel.disabled = job?.status === "canceling";
install.hidden = catalog?.available !== false;
source.hidden = !model;
if (model) source.href = model.source.startsWith("https://ollama.com/library/") ? model.source : "https://ollama.com/library/qwen2.5";
progress.hidden = !busy || !ownJob() || !job.total;
if (!progress.hidden) {
progress.max = job.total;
progress.value = Math.min(job.completed || 0, job.total);
}
if (errorMessage) status.textContent = errorMessage;
else if (busy && !ownJob()) status.textContent = `${copy.other} ${job.model} · ${job.endpoint}`;
else if (ownJob() && job.status === "downloading") {
const rawMsg = job.message ? ((document.documentElement.lang === "vi" && MSG_VI[job.message]) || job.message) : "";
const percent = job.total
? ` · ${copy.file}: ${Math.min(100, Math.floor(100 * (job.completed || 0) / job.total))}%`
: (rawMsg ? ` · ${rawMsg}` : "");
status.textContent = `${copy.downloading} ${job.model}${percent}`;
} else if (ownJob() && job.status === "canceling") status.textContent = copy.canceling;
else if (ownJob() && job.status === "failed") status.textContent = job.error || copy.failed;
else if (ownJob() && job.status === "canceled") status.textContent = copy.canceled;
else if (!catalog) status.textContent = copy.loading;
else if (!catalog.available) status.textContent = [copy.missing, catalog.error].filter(Boolean).join(" ");
else status.textContent = model?.installed ? copy.ready : "";
}

async function loadCatalog() {
const request = ++catalogRequest;
const response = await api(`/api/ai/local-models?endpoint=${encodeURIComponent(endpoint)}`);
if (!current() || request !== catalogRequest) return;
const selected = select.value || modelInput.value;
catalog = response;
select.replaceChildren(...catalog.models.map((model) => {
const option = node("option", `${model.name} · ${sizeLabel(model.size_bytes)}${model.installed ? ` · ${copy.ready}` : ""}`);
option.value = model.id;
return option;
}));
if (catalog.models.some((model) => model.id === selected)) select.value = selected;
render();
}

async function poll() {
clearTimeout(timer);
if (!current()) return;
const request = ++statusRequest;
try {
const response = await api("/api/ai/local-models/status");
if (!current() || request !== statusRequest) return;
errorMessage = "";
const finished = response.status === "done" && (job?.id !== response.id || job.status !== "done");
job = response;
if (finished && ownJob()) await loadCatalog();
if (!current()) return;
render();
if (inProgress(job)) {
if (Date.now() - started < 3_600_000) timer = setTimeout(poll, 1500);
else status.textContent = copy.timeout;
}
} catch (error) {
if (current() && request === statusRequest) { errorMessage = error.message; render(); }
}
}

select.onchange = () => { errorMessage = ""; render(); };
download.onclick = async () => {
const model = catalog?.models.find((entry) => entry.id === select.value);
if (!current() || !model || downloading || (!model.installed && inProgress(job))) return;
if (model.installed) {
modelInput.value = model.id;
status.textContent = copy.selected;
return;
}
errorMessage = "";
statusRequest += 1;
clearTimeout(timer);
downloading = true; render();
try {
const response = await api("/api/ai/local-models/download", {
method: "POST", body: JSON.stringify({model: model.id, endpoint}),
});
if (!current()) return;
job = response;
if (job.status === "done" && ownJob()) await loadCatalog();
if (!current()) return;
render();
await poll();
} catch (error) {
if (current()) errorMessage = error.message;
} finally {
downloading = false;
if (current()) render();
}
};
cancel.onclick = async () => {
if (!current() || !inProgress(job) || cancel.disabled) return;
errorMessage = "";
statusRequest += 1;
clearTimeout(timer);
cancel.disabled = true;
try {
const response = await api("/api/ai/local-models/cancel", {
method: "POST", body: JSON.stringify({id: job.id}),
});
if (!current()) return;
job = response;
render();
await poll();
} catch (error) {
if (current()) { errorMessage = error.message; render(); cancel.disabled = false; }
}
};
refresh.onclick = async () => {
refresh.disabled = true;
errorMessage = "";
try { await loadCatalog(); await poll(); }
catch (error) { if (current()) { errorMessage = error.message; render(); } }
finally { refresh.disabled = false; }
};
render();
loadCatalog().catch((error) => {
if (current()) { errorMessage = error.message; render(); }
});
poll();
}
