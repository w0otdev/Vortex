import assert from "node:assert/strict";
import {readFileSync} from "node:fs";

const source = readFileSync(new URL("../src/local_models.js", import.meta.url), "utf8");
const {open} = await import(`data:text/javascript;base64,${Buffer.from(source).toString("base64")}`);

class Element {
  constructor(tag = "div") {
    this.tag = tag;
    this.children = [];
    this.textContent = "";
    this.hidden = false;
    this.listeners = new Map();
    this.attributes = new Map();
  }
  get value() { return this.selected || (this.tag === "select" ? this.children[0]?.value : "") || ""; }
  set value(value) { this.selected = value; }
  append(...children) { this.children.push(...children); }
  replaceChildren(...children) {
    this.children = children;
    if (this.tag === "select") this.selected = "";
  }
  setAttribute(name, value) { this.attributes.set(name, value); }
  addEventListener(name, callback) { this.listeners.set(name, callback); }
  removeEventListener(name, callback) {
    if (this.listeners.get(name) === callback) this.listeners.delete(name);
  }
}

const originalDocument = globalThis.document;
const originalSetTimeout = globalThis.setTimeout;
const originalClearTimeout = globalThis.clearTimeout;
const originalNow = Date.now;
const timers = new Map();
let timerId = 0;
globalThis.setTimeout = (callback) => {timers.set(++timerId, callback); return timerId;};
globalThis.clearTimeout = (id) => timers.delete(id);
const settle = () => new Promise((resolve) => setImmediate(resolve));

function fixture() {
  const fields = new Map(["aiDialog", "aiLocalModels", "aiEndpoint", "aiModel", "aiProvider", "aiEnabled"].map((id) => [id, new Element()]));
  fields.get("aiDialog").open = true;
  fields.get("aiEndpoint").value = "http://127.0.0.1:11434/";
  fields.get("aiProvider").value = "ollama";
  fields.get("aiModel").value = "custom-local";
  fields.get("aiEnabled").checked = false;
  globalThis.document = {
    documentElement: {lang: "en"},
    getElementById: (id) => fields.get(id),
    createElement: (tag) => new Element(tag),
  };
  const models = [{id: "qwen2.5:0.5b", name: "Qwen2.5 0.5B", size_bytes: 398000000,
    source: "https://ollama.com/library/qwen2.5:0.5b", installed: false}];
  let job = {id: 0, status: "idle"};
  let available = true;
  const calls = [];
  const api = async (path, options) => {
    const body = options ? JSON.parse(options.body) : null;
    calls.push({path, body});
    if (path.startsWith("/api/ai/local-models?")) return {models: structuredClone(models), available};
    if (path.endsWith("/download")) job = {id: 7, model: body.model, endpoint: body.endpoint, status: "downloading", completed: 10, total: 100};
    if (path.endsWith("/cancel")) job = {...job, status: "canceled"};
    return {...job};
  };
  const ui = () => {
    const children = fields.get("aiLocalModels").children;
    return {select: children[1], source: children[3], install: children[4], download: children[5]?.children[0],
      cancel: children[5]?.children[1], refresh: children[5]?.children[2], progress: children[6], status: children[7]};
  };
  return {fields, models, calls, api, ui, setJob(value) {job = value;}, setAvailable(value) {available = value;}};
}

try {
  const appSource = readFileSync(new URL("../src/lib.rs", import.meta.url), "utf8");
  const initialBytes = ["INDEX_HTML", "STYLES_CSS", "APP_JS"].reduce((sum, name) => {
    const text = appSource.match(new RegExp(`pub const ${name}: &str = r##"([\\s\\S]*?)"##;`))[1];
    return sum + Buffer.byteLength(text);
  }, 0);
  assert.ok(initialBytes <= 153600, "Initial main window stays within 150KiB");
  assert.ok(!appSource.includes('<script src="/local-models.js"'), "Model UI loads only on demand");

  const disabled = fixture();
  disabled.fields.get("aiDialog").open = false;
  open(disabled.api);
  await settle();
  assert.equal(disabled.calls.length, 0, "Closed dialog makes no model requests");
  disabled.fields.get("aiDialog").open = true;
  disabled.fields.get("aiProvider").value = "openai";
  open(disabled.api);
  await settle();
  assert.equal(disabled.calls.length, 0, "Remote providers do not load Ollama catalog");

  const local = fixture();
  open(local.api);
  await settle();
  assert.match(local.ui().select.children[0].textContent, /398 MB/);
  assert.equal(local.ui().install.hidden, true);
  assert.equal(local.fields.get("aiModel").value, "custom-local", "Browsing presets preserves custom model");
  await local.ui().download.onclick();
  assert.deepEqual(local.calls.find((call) => call.path.endsWith("/download")).body,
    {model: "qwen2.5:0.5b", endpoint: "http://127.0.0.1:11434"});
  assert.equal(local.ui().progress.value, 10);
  assert.match(local.ui().status.textContent, /Current file: 10%/, "Progress is labeled as the current blob, not the complete model");
  assert.equal(local.fields.get("aiEnabled").checked, false, "Downloads never enable AI");
  assert.equal(timers.size, 1);
  local.models[0].installed = true;
  local.setJob({id: 7, model: "qwen2.5:0.5b", endpoint: "http://127.0.0.1:11434", status: "done"});
  await [...timers.values()][0]();
  assert.equal(local.ui().download.textContent, "Use model");
  assert.equal(local.fields.get("aiModel").value, "custom-local", "Completed download does not change settings");
  await local.ui().download.onclick();
  assert.equal(local.fields.get("aiModel").value, "qwen2.5:0.5b", "Use explicitly picks downloaded model");
  assert.equal(local.fields.get("aiEnabled").checked, false);
  assert.ok(!local.calls.some((call) => call.path === "/api/ai/settings"));

  const other = fixture();
  other.setJob({id: 12, model: "another-model", endpoint: "http://localhost:11435", status: "downloading"});
  open(other.api);
  await settle();
  assert.equal(other.ui().download.disabled, true, "One download remains the global limit");
  assert.equal(other.ui().cancel.hidden, false, "Existing download stays cancelable after endpoint change");
  assert.match(other.ui().status.textContent, /localhost:11435/);
  assert.equal(other.fields.get("aiModel").value, "custom-local");
  await other.ui().cancel.onclick();
  assert.deepEqual(other.calls.find((call) => call.path.endsWith("/cancel")).body, {id: 12});

  const closed = fixture();
  closed.setJob({id: 15, model: "qwen2.5:0.5b", endpoint: "http://127.0.0.1:11434", status: "downloading"});
  open(closed.api);
  await settle();
  closed.fields.get("aiDialog").open = false;
  closed.fields.get("aiDialog").listeners.get("close")();
  assert.equal(timers.size, 0, "Closing stops UI polling");
  assert.ok(!closed.calls.some((call) => call.path.endsWith("/cancel")), "Closing does not cancel a download");
  closed.fields.get("aiDialog").open = true;
  open(closed.api);
  await settle();
  assert.equal(timers.size, 1, "Reopening resumes progress");
  Date.now = () => originalNow() + 3_600_001;
  await [...timers.values()][0]();
  assert.equal(timers.size, 0, "UI polling has a ceiling");
  assert.match(closed.ui().status.textContent, /Reopen/);
  Date.now = originalNow;

  const missing = fixture();
  missing.setAvailable(false);
  open(missing.api);
  await settle();
  assert.equal(missing.ui().select.children.length, 1, "Catalog remains visible while Ollama is offline");
  assert.equal(missing.ui().install.hidden, false);
  assert.equal(missing.ui().download.disabled, true);

  const failed = fixture();
  open(async (path, options) => {
    if (path.endsWith("/download")) throw new Error("Download refused");
    return failed.api(path, options);
  });
  await settle();
  await failed.ui().download.onclick();
  assert.equal(failed.ui().status.textContent, "Download refused", "Download errors remain visible after button resets");
  assert.equal(failed.ui().download.disabled, false, "A failed request can be retried explicitly");

  const slowCatalog = fixture();
  let finishCatalog;
  slowCatalog.setJob({id: 21, model: "qwen2.5:0.5b", endpoint: "http://127.0.0.1:11434", status: "downloading"});
  open((path, options) => path.includes("?endpoint=")
    ? new Promise((resolve) => {finishCatalog = resolve;}) : slowCatalog.api(path, options));
  await settle();
  assert.equal(slowCatalog.ui().cancel.hidden, false, "A slow installed-model check never delays cancellation controls");
  finishCatalog({models: slowCatalog.models, available: true});
  await settle();

  const stale = fixture();
  let finish;
  open((path, options) => path.includes("?endpoint=")
    ? new Promise((resolve) => {finish = resolve;}) : stale.api(path, options));
  stale.fields.get("aiEndpoint").value = "http://localhost:11435";
  finish({models: stale.models, available: true});
  await settle();
  assert.equal(stale.ui().select.children.length, 0, "Old endpoint response cannot populate a new endpoint");
  console.log(`Local models UI: lazy load, consent, download/use, cancel, endpoint races, progress ceiling passed. Initial UI ${initialBytes}/153600 bytes.`);
} finally {
  globalThis.document = originalDocument;
  globalThis.setTimeout = originalSetTimeout;
  globalThis.clearTimeout = originalClearTimeout;
  Date.now = originalNow;
}
