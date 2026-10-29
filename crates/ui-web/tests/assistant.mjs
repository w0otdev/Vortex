import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import vm from "node:vm";

const source = readFileSync(new URL("../src/lib.rs", import.meta.url), "utf8");
const app = source.match(/pub const APP_JS: &str = r##"([\s\S]*?)"##;/)[1];
new vm.Script(app);

function load(name, context) {
  const match = app.match(new RegExp(`^  (?:async )?function ${name}\\([\\s\\S]*?^  }\\r?$`, "m"));
  assert.ok(match, `Function ${name} exists`);
  vm.runInContext(match[0], context);
}

const elements = new Map();
function element(id) {
  if (!elements.has(id)) elements.set(id, {
    textContent: "",
    classList: {
      values: new Set(),
      add(value) { this.values.add(value); },
      toggle(value, enabled) { enabled ? this.values.add(value) : this.values.delete(value); }
    }
  });
  return elements.get(id);
}
let callback;
const context = vm.createContext({
  $: element,
  autoSortEnabled: true,
  aiSettings: { enabled: false, accounts: ["allowed"] },
  aiSummaryId: 5,
  aiSummaryTimer: 0,
  activeCategory: "",
  selectedId: 5,
  api: async () => ({ status: "pending" }),
  setTimeout(fn) { callback = fn; return 1; },
  clearTimeout() { callback = null; },
  t: (key) => key
});
for (const name of ["sortItemsList", "updateAiReader", "updateAiKeyField", "pollAiSummary"]) load(name, context);

context.updateAiReader({ id: 5, account: "allowed" });
assert.ok(element("btnAiSummary").classList.values.has("hidden"), "AI remains off by default");
context.aiSettings.enabled = true;
context.updateAiReader({ id: 5, account: "other" });
assert.ok(element("btnAiSummary").classList.values.has("hidden"), "Non-opted mailbox stays excluded");
context.updateAiReader({ id: 5, account: "allowed" });
assert.ok(!element("btnAiSummary").classList.values.has("hidden"));
context.activeCategory = "__hidden";
context.updateAiReader({ id: 5, account: "allowed" });
assert.ok(!element("btnAiRestore").classList.values.has("hidden"), "Hidden messages remain recoverable");
context.updateAiReader({ id: 5, account: "allowed", summary: "Stored summary" });
assert.equal(element("readerAiSummary").textContent, "ai_summary_label: Stored summary", "Stored summary appears without requesting AI");
assert.ok(!element("readerAiSummary").classList.values.has("hidden"));
context.updateAiReader({ id: 5, account: "other", summary: "Excluded summary" });
assert.ok(element("readerAiSummary").classList.values.has("hidden"));
for (const provider of ["ollama", "llamacpp", "openai", "anthropic"]) {
  element("aiProvider").value = provider;
  context.updateAiKeyField();
  const local = provider === "ollama" || provider === "llamacpp";
  assert.equal(element("aiKey").disabled, local);
  assert.equal(element("aiKeyField").classList.values.has("hidden"), local);
}

const mail = (id, extra = {}) => ({ id, state: "unread", received_at: id, category: "work", ...extra });
const sorted = context.sortItemsList([
  mail(1, { priority_score: 90 }), mail(2, { priority_score: 10 }),
  mail(3, { category: "code_otp" }), mail(4, { pinned: true })
]);
assert.deepEqual(Array.from(sorted, (it) => it.id), [4, 3, 1, 2], "Pin and OTP precede AI priority");

context.aiSummaryId = 5;
await context.pollAiSummary(5);
assert.equal(typeof callback, "function", "Only pending result schedules a poll");
callback = null;
await context.pollAiSummary(5, 60);
assert.equal(callback, null, "Polling stops at its ceiling");
assert.equal(element("readerAiSummary").textContent, "ai_timeout");

context.api = async () => ({ status: "done", summary: "<img src=x onerror=alert(1)>" });
await context.pollAiSummary(5);
assert.equal(element("readerAiSummary").textContent, "ai_summary_label: <img src=x onerror=alert(1)>", "AI output is text, never HTML");
let complete;
context.api = () => new Promise((resolve) => { complete = resolve; });
const pending = context.pollAiSummary(5);
context.selectedId = 6;
element("readerAiSummary").textContent = "new selection";
complete({ status: "done", summary: "stale reply" });
await pending;
assert.equal(element("readerAiSummary").textContent, "new selection", "Stale result cannot overwrite another message");

console.log("AI UI: syntax, opt-in, restore, sorting, polling ceiling, escaping, selection races passed.");
