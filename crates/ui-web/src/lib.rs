pub const STYLES_CSS: &str = r##"/* hallmark · v2.0.0 · shape: app screen · genre: modern-minimal · theme: paper-ink (custom) · signature: live-otp-banner */
/* critique: calm two-pane casual inbox with centered single-purpose onboarding card and Heroicons 20/solid glyphs */
/* waive: custom-app-workbench */

@font-face {
  font-family: "Vortex SF Pro";
  src: local("SF Pro Display Bold"), local("SFProDisplay-Bold");
  font-style: normal;
  font-weight: 700;
  font-display: swap;
}

:root {
  --bg-page: #e4e4e0;
  --bg-surface: #f4f4f0;
  --bg-elevated: #fafaf7;
  --bg-subtle: #d6d6d0;
  --bg-inverse: #1a1a18;
  --bg-alert: #f2dede;
  --bg-notice: #e6e6e1;
  --bg-backdrop: rgba(24, 24, 22, 0.55);
  --text-primary: #181816;
  --text-secondary: #383835;
  --text-muted: #5c5c57;
  --text-inverse: #f4f4f0;
  --text-danger: #8f2424;
  --text-accent: #181816;
  --border-subtle: #94948b;
  --border-strong: #1a1a18;
  --border-danger: #b83c3c;
  --font-sans: "Vortex SF Pro", -apple-system, system-ui, "Segoe UI", Roboto, sans-serif;
  --font-mono: var(--font-sans);
  --radius-base: 10px;
  --container-max: 1280px;
  --col-gap: 16px;
  --glass-surface: #f4f4f0;
  --glass-edge: #9e9e96;
  --bevel-top: #fcfcf9;
  --bevel-bottom: #dcdcd4;
  --pill-ledge: #8c8c84;
  --pill-ledge-strong: #0e0e0d;
  --shadow-ambient: rgba(24, 24, 22, 0.12);
  --glass-shadow: inset 0 1.5px 0 var(--bevel-top), inset 0 -1px 0 var(--bevel-bottom), 0 3px 0 var(--pill-ledge), 0 7px 14px var(--shadow-ambient);
  --btn-3d-shadow: inset 0 1.5px 0 var(--bevel-top), inset 0 -1px 0 var(--bevel-bottom), 0 3px 0 var(--pill-ledge), 0 4px 8px var(--shadow-ambient);
  --btn-3d-hover-shadow: inset 0 1.5px 0 var(--bevel-top), inset 0 -1px 0 var(--bevel-bottom), 0 4px 0 var(--pill-ledge), 0 7px 12px var(--shadow-ambient);
  --btn-3d-active-shadow: inset 0 1px 0 var(--bevel-bottom), 0 1px 0 var(--pill-ledge), 0 2px 4px var(--shadow-ambient);
  --btn-primary-shadow: inset 0 1.5px 0 rgba(244, 244, 240, 0.22), inset 0 -1px 0 rgba(14, 14, 13, 0.5), 0 3px 0 var(--pill-ledge-strong), 0 5px 10px var(--shadow-ambient);
  --btn-primary-active-shadow: inset 0 1px 0 rgba(14, 14, 13, 0.6), 0 1px 0 var(--pill-ledge-strong);
  --glider-surface: #1c1c1a;
  --glider-text: #f4f4f0;
  --glider-edge: #121210;
  --glider-shadow: inset 0 1.5px 0 rgba(244, 244, 240, 0.24), inset 0 -1px 0 rgba(14, 14, 13, 0.6), 0 2.5px 0 #0e0e0d, 0 5px 10px rgba(24, 24, 22, 0.18);
  --glass-hover: rgba(24, 24, 22, 0.07);
  --glass-selected: #dfdfd9;
  --row-border: #b0b0a6;
  --row-hover: #eaeae4;
  --row-selected: #dcdcd4;
  --scrollbar-thumb: #9e9e95;
  --scrollbar-thumb-hover: #62625b;
  --scrollbar-thumb-active: #1a1a18;
}

[data-theme="dark"] {
  --bg-page: #121211;
  --bg-surface: #1b1b19;
  --bg-elevated: #232320;
  --bg-subtle: #282825;
  --bg-inverse: #ecece6;
  --bg-alert: #2e1b1b;
  --bg-notice: #242421;
  --bg-backdrop: rgba(12, 12, 11, 0.72);
  --text-primary: #ecece6;
  --text-secondary: #bdbdb6;
  --text-muted: #8a8a82;
  --text-inverse: #161614;
  --text-danger: #e07070;
  --text-accent: #ecece6;
  --border-subtle: #4a4a44;
  --border-strong: #ecece6;
  --border-danger: #b84a4a;
  --glass-surface: #20201e;
  --glass-edge: #3e3e39;
  --bevel-top: rgba(236, 236, 230, 0.14);
  --bevel-bottom: rgba(11, 11, 10, 0.65);
  --pill-ledge: #0c0c0b;
  --pill-ledge-strong: #8e8e86;
  --shadow-ambient: rgba(10, 10, 9, 0.55);
  --glass-shadow: inset 0 1.5px 0 var(--bevel-top), inset 0 -1px 0 var(--bevel-bottom), 0 3px 0 var(--pill-ledge), 0 8px 16px var(--shadow-ambient);
  --btn-3d-shadow: inset 0 1.5px 0 var(--bevel-top), inset 0 -1px 0 var(--bevel-bottom), 0 3px 0 var(--pill-ledge), 0 5px 10px var(--shadow-ambient);
  --btn-3d-hover-shadow: inset 0 1.5px 0 var(--bevel-top), inset 0 -1px 0 var(--bevel-bottom), 0 4px 0 var(--pill-ledge), 0 8px 14px var(--shadow-ambient);
  --btn-3d-active-shadow: inset 0 1px 0 var(--bevel-bottom), 0 1px 0 var(--pill-ledge), 0 2px 4px var(--shadow-ambient);
  --btn-primary-shadow: inset 0 1.5px 0 #fafaf5, inset 0 -1px 0 #c8c8c0, 0 3px 0 var(--pill-ledge-strong), 0 6px 12px var(--shadow-ambient);
  --btn-primary-active-shadow: inset 0 1px 0 #b8b8b0, 0 1px 0 var(--pill-ledge-strong);
  --glider-surface: #ecece6;
  --glider-text: #161614;
  --glider-edge: #f4f4ee;
  --glider-shadow: inset 0 1.5px 0 #fafaf5, inset 0 -1px 0 #c6c6be, 0 2.5px 0 #8a8a82, 0 5px 12px rgba(10, 10, 9, 0.55);
  --glass-hover: rgba(236, 236, 230, 0.08);
  --glass-selected: #2f2f2b;
  --row-border: #3a3a35;
  --row-hover: #232320;
  --row-selected: #2d2d29;
  --scrollbar-thumb: #4e4e48;
  --scrollbar-thumb-hover: #7c7c74;
  --scrollbar-thumb-active: #ecece6;
}

*, *::before, *::after { box-sizing: border-box; }
*::-webkit-scrollbar { width: 8px; height: 0 !important; background: transparent; }
*::-webkit-scrollbar:horizontal, *::-webkit-scrollbar-button, *::-webkit-scrollbar-corner { display: none !important; width: 0 !important; height: 0 !important; background: transparent; }
*::-webkit-scrollbar-track { background: transparent; margin-block: 6px; }
*::-webkit-scrollbar-thumb { background-color: var(--scrollbar-thumb); border-radius: 999px; border: 2px solid transparent; background-clip: padding-box; min-height: 36px; }
*::-webkit-scrollbar-thumb:hover { background-color: var(--scrollbar-thumb-hover); }
*::-webkit-scrollbar-thumb:active { background-color: var(--scrollbar-thumb-active); }
html, body { margin: 0; padding: 0; height: 100vh; overflow-x: clip; overflow-y: hidden; background-color: var(--bg-page); color: var(--text-primary); font-family: var(--font-sans); font-weight: 700; font-size: 16px; line-height: 1.5; }
h1, h2, h3 { margin: 0; font-weight: 700; color: var(--text-primary); overflow-wrap: break-word; min-width: 0; letter-spacing: 0; }
p { margin: 0; }
button, input, select, textarea { font-family: var(--font-sans); font-weight: 700; font-size: 0.9375rem; color: var(--text-primary); }
:focus-visible { outline: 2px solid var(--text-accent); outline-offset: 2px; }
.svg-sprite { position: absolute; width: 0; height: 0; overflow: hidden; pointer-events: none; }
.hi-icon { width: 18px; height: 18px; flex-shrink: 0; fill: currentColor; display: inline-block; }
.hi-icon-lg { width: 22px; height: 22px; flex-shrink: 0; fill: currentColor; }
@keyframes spin { to { transform: rotate(360deg); } }
@keyframes rowFadeUp { 0% { opacity: 0; transform: translate3d(0, 8px, 0); } 100% { opacity: 1; transform: translate3d(0, 0, 0); } }
@keyframes readerStageIn { 0% { opacity: 0; transform: translate3d(0, 12px, 0); } 100% { opacity: 1; transform: translate3d(0, 0, 0); } }
@keyframes readerCanvasUnfold { 0% { opacity: 0; transform: translate3d(0, 16px, 0); } 100% { opacity: 1; transform: translate3d(0, 0, 0); } }
@keyframes panelEnter { from { opacity: 0; transform: translateY(12px); } to { opacity: 1; transform: translateY(0); } }
.is-spinning .hi-icon { animation: spin 700ms linear infinite; }
@keyframes syncPop { 0% { transform: scale(0.55) rotate(-18deg); opacity: 0; } 55% { transform: scale(1.18) rotate(0deg); opacity: 1; } 100% { transform: scale(1) rotate(0deg); opacity: 1; } }
.topbar .refresh-btn.is-synced { background: var(--bg-inverse); color: var(--text-inverse); border-color: var(--border-strong); box-shadow: var(--btn-primary-shadow); }
.topbar .refresh-btn.is-synced .hi-icon { animation: syncPop 380ms cubic-bezier(0.22, 1, 0.36, 1) forwards; }
.shell { max-width: var(--container-max); margin-inline: auto; height: 100vh; display: flex; flex-direction: column; overflow: hidden; padding: 14px 20px 14px; }
.topbar { position: relative; z-index: 10; flex-shrink: 0; display: grid; grid-template-columns: auto minmax(200px, 1fr) auto auto auto; align-items: center; gap: 12px; padding: 4px 0 12px; -webkit-user-select: none; user-select: none; }
.window-controls { display: inline-flex; align-items: center; gap: 4px; padding: 5px; }
.topbar .win-btn { width: 38px; min-width: 38px; padding: 0; justify-content: center; }
.topbar .win-btn-close:hover { background: var(--bg-alert); color: var(--text-danger); border-color: var(--border-danger); }
.glass-pill { background: var(--glass-surface); border: 1px solid var(--glass-edge); border-radius: 999px; box-shadow: var(--glass-shadow); -webkit-backdrop-filter: blur(24px) saturate(140%); backdrop-filter: blur(24px) saturate(140%); transition: transform 160ms cubic-bezier(0.22, 1, 0.36, 1), box-shadow 160ms ease, background-color 160ms ease, border-color 160ms ease; }
.brand-mark { display: inline-flex; align-items: center; gap: 10px; color: var(--text-accent); white-space: nowrap; flex-shrink: 0; }
.brand-mark > .hi-icon-lg { width: 28px; height: 28px; color: var(--text-primary); }
.brand-title { font-size: 1.125rem; letter-spacing: -0.035em; }
.search-wrap { position: relative; display: flex; align-items: center; min-width: 0; padding: 4px; }
.search-wrap > .hi-icon { position: absolute; left: 16px; top: 50%; transform: translateY(-50%); color: var(--text-muted); pointer-events: none; transition: color 150ms ease; }
.search-wrap:focus-within > .hi-icon { color: var(--text-accent); }
.input-field { width: 100%; min-height: 40px; padding: 8px 14px; background-color: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-base); box-shadow: inset 0 2px 4px var(--shadow-ambient); transition: border-color 150ms ease, box-shadow 150ms ease; }
.input-field:focus { border-color: var(--text-accent); box-shadow: 0 0 0 3px var(--glass-hover); outline: none; }
.search-wrap .input-field { min-width: 0; min-height: 40px; padding: 8px 12px 8px 40px; background: transparent; border: 0; border-radius: 999px; font-size: 0.875rem; box-shadow: none; }
.search-wrap:focus-within { outline: 2px solid var(--text-accent); outline-offset: 2px; }
.mailbox-controls { display: flex; align-items: center; min-width: 0; gap: 6px; padding: 5px; }
.topbar .btn { min-height: 38px; padding-inline: 13px; border: 1px solid transparent; border-radius: 999px; background: transparent; box-shadow: none; font-size: 0.8125rem; font-weight: 700; }
.topbar .btn:hover { background: var(--glass-selected); border-color: var(--glass-edge); box-shadow: inset 0 1px 0 var(--bevel-top), 0 2px 0 var(--pill-ledge); transform: translateY(-1px); }
.topbar .btn:active { transform: translateY(1px); box-shadow: inset 0 1px 2px var(--shadow-ambient); }
.topbar .refresh-btn { flex-shrink: 0; width: 38px; padding: 0; color: var(--text-accent); background: var(--bg-elevated); border: 1px solid var(--glass-edge); box-shadow: inset 0 1px 0 var(--bevel-top), 0 2px 0 var(--pill-ledge); }
.topbar .refresh-btn:hover { background: var(--glass-selected); transform: translateY(-1px); box-shadow: inset 0 1px 0 var(--bevel-top), 0 3px 0 var(--pill-ledge); }
.topbar .refresh-btn:active { transform: translateY(1.5px); box-shadow: 0 0.5px 0 var(--pill-ledge); }
.topbar .btn-primary { background: var(--bg-inverse); color: var(--text-inverse); border: 1px solid var(--border-strong); box-shadow: var(--btn-primary-shadow); }
.topbar .btn-primary:hover { background: var(--bg-inverse); border-color: var(--border-strong); box-shadow: var(--btn-primary-shadow); opacity: 0.94; transform: translateY(-1px); }
.topbar .btn-primary:active { transform: translateY(2px); box-shadow: var(--btn-primary-active-shadow); }
.language-controls { display: inline-flex; align-items: center; gap: 4px; padding: 5px; }
#btnTheme { min-width: 82px; justify-content: center; }
#btnTheme .hi-icon { transition: transform 360ms cubic-bezier(0.34, 1.56, 0.64, 1); }
#btnTheme.is-flipping .hi-icon { transform: rotate(180deg) scale(1.15); }
::view-transition-old(root), ::view-transition-new(root) { animation: none; mix-blend-mode: normal; }
::view-transition-new(root) { z-index: 9999; }
html.theme-transitioning, html.theme-transitioning *, html.theme-transitioning *::before, html.theme-transitioning *::after { transition: background-color 260ms cubic-bezier(0.22, 1, 0.36, 1), color 220ms cubic-bezier(0.22, 1, 0.36, 1), border-color 260ms cubic-bezier(0.22, 1, 0.36, 1), box-shadow 260ms cubic-bezier(0.22, 1, 0.36, 1) !important; }
#btnLang { min-width: 66px; justify-content: center; perspective: 260px; }
#btnLang .hi-icon { transition: transform 440ms cubic-bezier(0.34, 1.56, 0.64, 1); transform-style: preserve-3d; }
#btnLang.is-switching .hi-icon { transform: rotateY(180deg) rotateZ(-10deg) scale(1.2); }
#langLabel { display: inline-block; transform-origin: 50% 50%; }
@keyframes langDrumRoll { 0% { opacity: 1; transform: translateY(0) rotateX(0deg) scale(1); filter: blur(0px); } 42% { opacity: 0; transform: translateY(-9px) rotateX(75deg) scale(0.84); filter: blur(2.5px); } 43% { opacity: 0; transform: translateY(9px) rotateX(-75deg) scale(0.84); filter: blur(2.5px); } 100% { opacity: 1; transform: translateY(0) rotateX(0deg) scale(1); filter: blur(0px); } }
#btnLang.is-switching #langLabel { animation: langDrumRoll 380ms cubic-bezier(0.22, 1, 0.36, 1) forwards; }
@keyframes i18nWaveOut { 0% { opacity: 1; transform: translateY(0) scale(1); filter: blur(0px); } 100% { opacity: 0; transform: translateY(-5px) scale(0.96); filter: blur(3px); } }
@keyframes i18nWaveIn { 0% { opacity: 0; transform: translateY(6px) scale(0.96); filter: blur(3px); } 100% { opacity: 1; transform: translateY(0) scale(1); filter: blur(0px); } }
.i18n-wave-out { animation: i18nWaveOut 130ms cubic-bezier(0.4, 0, 1, 1) forwards; animation-delay: var(--i18n-delay, 0ms); }
.i18n-wave-in { animation: i18nWaveIn 260ms cubic-bezier(0.22, 1, 0.36, 1) both; animation-delay: var(--i18n-delay, 0ms); }
.topbar .acct-trigger { max-width: 210px; background: var(--bg-elevated); border: 1px solid var(--glass-edge); box-shadow: inset 0 1px 0 var(--bevel-top), 0 2px 0 var(--pill-ledge); }
.acct-chevron { width: 12px; height: 12px; flex-shrink: 0; transition: transform 180ms ease; }
.acct-trigger[aria-expanded="true"] .acct-chevron { transform: rotate(180deg); }

.btn { appearance: none; display: inline-flex; align-items: center; justify-content: center; gap: 8px; min-height: 38px; padding: 0 14px; background-color: var(--bg-elevated); color: var(--text-primary); border: 1px solid var(--glass-edge); border-radius: 999px; box-shadow: var(--btn-3d-shadow); white-space: nowrap; cursor: pointer; transition: transform 140ms cubic-bezier(0.22, 1, 0.36, 1), box-shadow 140ms ease, background-color 140ms ease, border-color 140ms ease, opacity 140ms ease; }
.btn:hover { background-color: var(--glass-selected); box-shadow: var(--btn-3d-hover-shadow); transform: translateY(-1.5px); }
.btn:active { transform: translateY(2px); box-shadow: var(--btn-3d-active-shadow); }
.btn:disabled { opacity: 0.5; cursor: not-allowed; }
.btn-primary { background-color: var(--bg-inverse); color: var(--text-inverse); border-color: var(--border-strong); box-shadow: var(--btn-primary-shadow); font-weight: 700; }
.btn-primary:hover { opacity: 0.94; background-color: var(--bg-inverse); box-shadow: var(--btn-primary-shadow); }
.btn-primary:active { transform: translateY(2px); box-shadow: var(--btn-primary-active-shadow); }
.btn-large { width: 100%; min-height: 44px; font-size: 1rem; }
.btn-google { width: 100%; min-height: 46px; margin-top: 18px; font-size: 1rem; font-weight: 700; background-color: var(--bg-inverse); color: var(--text-inverse); border-color: var(--border-strong); box-shadow: var(--btn-primary-shadow); }
.btn-google:hover { opacity: 0.94; background-color: var(--bg-inverse); box-shadow: var(--btn-primary-shadow); }
.btn-google:active { transform: translateY(2px); box-shadow: var(--btn-primary-active-shadow); }
.divider-caption { margin-top: 18px; padding-top: 14px; border-top: 1px solid var(--border-subtle); font-size: 0.8125rem; color: var(--text-muted); }
.btn-danger { background-color: var(--bg-alert); color: var(--text-danger); border-color: var(--border-danger); }
.btn-ghost { background-color: transparent; border-color: transparent; box-shadow: none; color: var(--text-secondary); }
.btn-ghost:hover { background-color: var(--bg-subtle); color: var(--text-primary); }
.otp-meta { font-size: .8125rem; color: var(--text-secondary); }
.onboarding-wrap { width: 100%; max-width: 640px; max-height: calc(100vh - 96px); overflow-y: auto; margin: 16px auto 0; background-color: var(--bg-surface); border: 1px solid var(--glass-edge); border-radius: 20px; box-shadow: var(--glass-shadow); padding: 24px 28px; animation: panelEnter 240ms cubic-bezier(0.22, 1, 0.36, 1); }
.onboarding-head { display: flex; align-items: flex-start; justify-content: space-between; gap: 12px; }
.onboarding-title { font-size: 1.375rem; }
.onboarding-sub { margin-top: 6px; font-size: 0.9375rem; color: var(--text-secondary); }
.mode-switch { margin-top: 18px; display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 10px; }
.provider-grid { margin-top: 16px; display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 10px; }

.preset-btn { appearance: none; display: flex; align-items: center; justify-content: center; gap: 8px; min-height: 42px; padding: 0 14px; background-color: var(--bg-elevated); color: var(--text-primary); border: 1px solid var(--glass-edge); border-radius: 999px; box-shadow: var(--btn-3d-shadow); font-size: 0.9375rem; font-weight: 700; cursor: pointer; transition: transform 140ms cubic-bezier(0.22, 1, 0.36, 1), box-shadow 140ms ease, background-color 140ms ease, color 140ms ease, border-color 140ms ease; }
.preset-btn:hover { background-color: var(--glass-selected); box-shadow: var(--btn-3d-hover-shadow); transform: translateY(-1.5px); }
.preset-btn:active { transform: translateY(2px); box-shadow: var(--btn-3d-active-shadow); }
.preset-btn[aria-pressed="true"] { background-color: var(--bg-inverse); color: var(--text-inverse); border-color: var(--border-strong); box-shadow: var(--btn-primary-shadow); font-weight: 700; }
.preset-btn[aria-pressed="true"]:active { transform: translateY(2px); box-shadow: var(--btn-primary-active-shadow); }
.form-stack { margin-top: 16px; display: flex; flex-direction: column; gap: 14px; }
.bulk-textarea { min-height: 120px; font-family: var(--font-mono); font-size: 0.875rem; line-height: 1.5; resize: vertical; }
.field-label { display: block; font-size: 0.875rem; font-weight: 700; color: var(--text-primary); margin-bottom: 6px; }
.field-hint { margin-top: 6px; font-size: 0.8125rem; color: var(--text-secondary); }
.secret-input-group { display: flex; gap: 8px; }
.server-grid { display: grid; grid-template-columns: 2fr 1fr; gap: 12px; }
.acct-switcher { position: relative; display: inline-flex; }
.acct-trigger { max-width: 240px; }
.acct-active-label { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.acct-menu { position: absolute; top: calc(100% + 6px); right: 0; width: min(340px, calc(100dvw - 40px)); max-height: 400px; overflow-y: auto; background-color: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: 24px; padding: 10px; z-index: 850; box-shadow: 0 12px 32px rgba(24, 24, 22, 0.14); animation: panelEnter 180ms cubic-bezier(0.16, 1, 0.3, 1); }
.acct-menu-list { display: flex; flex-direction: column; gap: 2px; }
.acct-item-wrap { display: flex; align-items: center; gap: 4px; border-radius: 6px; padding-right: 4px; transition: background-color 140ms ease; }
.acct-item-wrap:hover, .acct-item-wrap[data-active="true"] { background-color: var(--bg-subtle); }
.acct-item-btn { flex: 1; min-width: 0; display: flex; align-items: center; justify-content: space-between; gap: 10px; padding: 8px 10px; background: transparent; border: none; border-radius: 6px; color: var(--text-primary); font-family: inherit; font-size: 0.875rem; text-align: left; cursor: pointer; }
.acct-item-wrap[data-active="true"] .acct-item-btn { font-weight: 700; }
.acct-item-main { display: flex; align-items: center; gap: 8px; min-width: 0; }
.acct-item-text { display: flex; flex-direction: column; min-width: 0; }
.acct-item-email { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; line-height: 1.3; }
.acct-item-sub { font-size: 0.75rem; font-weight: 700; color: var(--text-muted); line-height: 1.3; }
.acct-item-del { flex-shrink: 0; width: 28px; height: 28px; display: inline-flex; align-items: center; justify-content: center; background: transparent; border: none; border-radius: 6px; color: var(--text-muted); cursor: pointer; transition: color 140ms ease, background-color 140ms ease; }
.acct-item-del:hover { color: var(--status-danger); background-color: var(--status-danger-bg); }
.acct-menu-foot { margin-top: 6px; padding-top: 6px; border-top: 1px solid var(--border-subtle); display: flex; align-items: center; justify-content: space-between; gap: 6px; }
.account-row-actions { display: flex; flex-wrap: wrap; align-items: center; gap: 6px; }
.btn-sm { min-height: 32px; padding: 0 10px; font-size: 0.8125rem; }
#inboxView { flex: 1 1 0%; min-height: 0; display: flex; flex-direction: column; overflow: hidden; }
@keyframes catIconSpring { 0% { transform: scale(0.78) rotate(-8deg); } 55% { transform: scale(1.14) rotate(3deg); } 100% { transform: scale(1) rotate(0deg); } }
@keyframes catStreamMorph { 0% { opacity: 0; transform: translate3d(0, 6px, 0); } 100% { opacity: 1; transform: translate3d(0, 0, 0); } }
.filter-bar { flex-shrink: 0; margin-top: 4px; display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 8px; }
.filter-bar-right { display: flex; align-items: center; gap: 10px; margin-left: auto; min-width: 0; }
.filter-bar-right .sort-btn { border-radius: 999px; border-color: var(--glass-edge); background: var(--bg-elevated); box-shadow: var(--btn-3d-shadow); min-height: 46px; padding-inline: 18px; font-size: 0.8125rem; font-weight: 700; }
.filter-bar-right .sort-btn:hover { background-color: var(--glass-selected); box-shadow: var(--btn-3d-hover-shadow); transform: translateY(-1.5px); }
.filter-bar-right .sort-btn:active { transform: translateY(2px); box-shadow: var(--btn-3d-active-shadow); }
#statusNotice { display: none !important; }
.sort-btn[aria-pressed="true"] { background-color: var(--bg-inverse); color: var(--text-inverse); border-color: var(--border-strong); box-shadow: var(--btn-primary-shadow); font-weight: 700; }
.sort-btn[aria-pressed="true"]:hover { background-color: var(--bg-inverse); color: var(--text-inverse); box-shadow: var(--btn-primary-shadow); }
.sort-btn[aria-pressed="true"]:active { transform: translateY(2px); box-shadow: var(--btn-primary-active-shadow); }
.sort-btn[aria-pressed="true"] .hi-icon { color: var(--text-inverse); animation: catIconSpring 460ms cubic-bezier(0.16, 1, 0.3, 1); }
.category-pills { position: relative; display: flex; flex-wrap: nowrap; align-items: center; gap: 5px; max-width: 100%; min-width: 0; padding: 5px; overflow-x: auto; overscroll-behavior-x: contain; scrollbar-width: none; }
.category-pills::-webkit-scrollbar { display: none; }
.cat-glider { position: absolute; top: 0; left: 0; width: 0; height: 0; background-color: var(--glider-surface); border: 1px solid var(--glider-edge); border-radius: 999px; box-shadow: var(--glider-shadow); pointer-events: none; z-index: 0; opacity: 0; transition: transform 420ms cubic-bezier(0.16, 1, 0.3, 1), width 420ms cubic-bezier(0.16, 1, 0.3, 1), height 420ms cubic-bezier(0.16, 1, 0.3, 1), opacity 200ms ease; }
.cat-btn { position: relative; z-index: 1; appearance: none; display: inline-flex; align-items: center; gap: 6px; flex-shrink: 0; min-height: 38px; padding: 0 13px; background-color: transparent; color: var(--text-secondary); border: 1px solid transparent; border-radius: 999px; font-size: 0.8125rem; font-weight: 700; white-space: nowrap; cursor: pointer; transition: transform 160ms cubic-bezier(0.16, 1, 0.3, 1), color 200ms ease, background-color 160ms ease, box-shadow 160ms ease, border-color 160ms ease; }
.cat-btn:hover { color: var(--text-primary); background-color: var(--glass-selected); border-color: var(--glass-edge); box-shadow: inset 0 1px 0 var(--bevel-top), 0 2px 0 var(--pill-ledge); transform: translateY(-1px); }
.cat-btn:active { transform: translateY(1.5px); box-shadow: 0 0.5px 0 var(--pill-ledge); }
.cat-btn[aria-pressed="true"] { color: var(--glider-text); font-weight: 700; }
.cat-btn[aria-pressed="true"]:hover { color: var(--glider-text); background-color: transparent; border-color: transparent; box-shadow: none; transform: none; }
.cat-btn.is-glider-fallback[aria-pressed="true"] { background-color: var(--glider-surface); color: var(--glider-text); border-color: var(--glider-edge); box-shadow: var(--glider-shadow); }
.cat-btn[aria-pressed="true"] .hi-icon { color: var(--glider-text); animation: catIconSpring 460ms cubic-bezier(0.16, 1, 0.3, 1); }
.cat-count { min-width: 18px; padding-inline: 5px; border-radius: 999px; background: var(--glass-selected); font-size: 0.75rem; font-weight: 700; font-variant-numeric: tabular-nums; color: var(--text-secondary); transition: color 200ms ease, background-color 200ms ease, transform 240ms cubic-bezier(0.16, 1, 0.3, 1); }
.cat-count:empty { display: none; }
.cat-btn[aria-pressed="true"] .cat-count { background: var(--bg-surface); color: var(--text-primary); font-weight: 700; }
.stream-list.is-cat-switching { animation: catStreamMorph 500ms cubic-bezier(0.16, 1, 0.3, 1) both; }
.inbox-grid { flex: 1 1 0%; min-height: 0; margin-top: 10px; display: grid; grid-template-columns: clamp(300px, 34vw, 390px) minmax(0, 1fr); grid-template-rows: minmax(0, 1fr); gap: var(--col-gap); align-items: stretch; overflow: hidden; }
.stream-pane { background-color: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-base); box-shadow: 0 2px 10px rgba(17, 17, 15, 0.05); height: 100%; min-height: 0; overflow-x: hidden; overflow-y: auto; overscroll-behavior: contain; scrollbar-gutter: stable; }
.reader-pane { background-color: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-base); box-shadow: 0 2px 10px rgba(17, 17, 15, 0.05); height: 100%; min-height: 0; overflow: hidden; display: flex; flex-direction: column; padding: 20px 24px 16px; }
.stream-list { list-style: none; margin: 0; padding: 0; overflow-x: clip; }
.stream-row { position: relative; padding: 13px 16px 13px 20px; border-bottom: 1px solid var(--row-border); overflow-x: clip; cursor: pointer; transition: background-color 280ms ease; }
.stream-list.is-cat-switching .stream-row { animation: rowFadeUp 460ms cubic-bezier(0.16, 1, 0.3, 1) both; animation-delay: calc(var(--row-idx, 0) * 32ms); }
.stream-row::before { content: ""; position: absolute; left: 6px; top: 12px; bottom: 12px; width: 3px; border-radius: 999px; background-color: var(--text-accent); opacity: 0; transform: scaleY(0.4); transition: transform 340ms cubic-bezier(0.16, 1, 0.3, 1), opacity 220ms ease; }
.stream-row:hover { background-color: var(--row-hover); }
.stream-row:last-child { border-bottom: none; }
.stream-row[aria-selected="true"] { background-color: var(--row-selected); }
.stream-row[aria-selected="true"]::before { opacity: 1; transform: scaleY(1); }
.stream-row-unread .row-sender-name, .stream-row-unread .row-subject { font-weight: 700; }
.stream-row-unread .sender-avatar { color: var(--text-accent); }
.row-header { display: flex; align-items: flex-start; gap: 10px; }
.sender-avatar { width: 34px; height: 34px; border-radius: 50%; flex-shrink: 0; display: inline-flex; align-items: center; justify-content: center; background-color: var(--bg-subtle); border: 1px solid var(--border-subtle); color: var(--text-primary); overflow: hidden; margin-top: 1px; }
.sender-avatar-img { width: 100%; height: 100%; object-fit: cover; display: block; }
.sender-avatar-initial { font-size: 0.875rem; font-weight: 700; color: var(--text-accent); line-height: 1; }
.sender-info { flex: 1 1 auto; min-width: 0; }
.sender-top-line { display: flex; align-items: baseline; justify-content: space-between; gap: 8px; }
.row-sender-name { font-size: 0.9rem; font-weight: 700; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0; }
.row-sender-email { font-size: 0.75rem; line-height: 1.25; color: var(--text-muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.row-time { flex-shrink: 0; white-space: nowrap; font-variant-numeric: tabular-nums; font-size: 0.75rem; color: var(--text-muted); }
.row-subject { margin-top: 6px; font-size: 0.90625rem; line-height: 1.35; color: var(--text-primary); display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
.row-snippet { margin-top: 3px; font-size: 0.8125rem; line-height: 1.42; color: var(--text-muted); display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
.stream-empty { padding: 48px 24px; text-align: center; color: var(--text-secondary); }
.stream-empty-title { margin-top: 10px; font-size: 1rem; font-weight: 700; color: var(--text-primary); }
.stream-empty-desc { margin-top: 6px; margin-bottom: 16px; font-size: 0.875rem; color: var(--text-secondary); }
.reader-empty { margin: auto; padding: 40px 24px; text-align: center; color: var(--text-muted); }
.reader-content-wrap { display: flex; flex-direction: column; flex: 1 1 0%; min-height: 0; }
.reader-animate #readerSubject { animation: readerStageIn 580ms cubic-bezier(0.16, 1, 0.3, 1) both; }
.reader-animate .reader-meta-row { animation: readerStageIn 620ms cubic-bezier(0.16, 1, 0.3, 1) 70ms both; }
.reader-animate .reader-actions { animation: readerStageIn 640ms cubic-bezier(0.16, 1, 0.3, 1) 130ms both; }
.reader-animate .reader-html-frame, .reader-animate .reader-body { transform-origin: top center; animation: readerCanvasUnfold 760ms cubic-bezier(0.16, 1, 0.3, 1) 170ms both; }
.mobile-back-btn { display: none; margin-bottom: 12px; align-self: flex-start; }

.reader-meta-row { margin-top: 10px; display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 8px 16px; font-size: 0.875rem; color: var(--text-secondary); }
.reader-sender-block { display: flex; align-items: center; gap: 10px; min-width: 0; }
.reader-sender-stack { display: flex; flex-direction: column; min-width: 0; }
.reader-sender-strong { font-weight: 700; font-size: 0.9375rem; color: var(--text-primary); line-height: 1.25; }
.reader-sender-sub { font-size: 0.78125rem; color: var(--text-muted); line-height: 1.3; }
.reader-actions { flex-shrink: 0; display: flex; flex-wrap: wrap; gap: 8px; margin-top: 12px; padding-bottom: 12px; border-bottom: 1px solid var(--border-subtle); }
.phishing-box { flex-shrink: 0; margin-top: 10px; padding: 10px 14px; background-color: var(--bg-alert); border: 1px solid var(--border-danger); border-radius: var(--radius-base); color: var(--text-danger); font-size: 0.875rem; display: flex; align-items: center; gap: 10px; }
.reader-html-frame { flex: 1 1 0%; min-height: 0; width: 100%; border: none; margin-top: 8px; background-color: var(--bg-surface); }
.reader-body { flex: 1 1 0%; min-height: 0; overflow-y: auto; overscroll-behavior: contain; margin-top: 14px; white-space: pre-wrap; overflow-wrap: break-word; font-size: 0.96875rem; line-height: 1.68; color: var(--text-primary); }
.toast-bar { position: fixed; right: 20px; bottom: 20px; z-index: 900; background-color: var(--bg-inverse); color: var(--text-inverse); padding: 10px 16px; border-radius: var(--radius-base); font-size: 0.875rem; font-weight: 700; display: inline-flex; align-items: center; gap: 8px; transform: translateY(16px); opacity: 0; pointer-events: none; transition: transform 200ms cubic-bezier(0.22, 1, 0.36, 1), opacity 200ms ease; }
.toast-bar.is-visible { transform: translateY(0); opacity: 1; }
.hidden { display: none !important; }
@media (max-width: 1100px) {
  .topbar { grid-template-columns: auto minmax(0, 1fr) auto auto; }
  .mailbox-controls { justify-self: end; }
  .search-wrap { order: 5; grid-column: 1 / -1; }
  .category-pills { flex: 1 1 0%; }
  .filter-bar { flex-wrap: nowrap; }
}
@media (max-width: 880px) {
  .shell { padding: 12px 14px 20px; }
  .inbox-grid { grid-template-columns: 280px minmax(0, 1fr); gap: 12px; }
  .reader-pane { padding: 18px 20px; }
}
@media (max-width: 720px) {
  .provider-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .onboarding-wrap { padding: 20px 18px; margin-top: 12px; }
  .inbox-grid { grid-template-columns: minmax(0, 1fr); }
  .mobile-back-btn { display: inline-flex; }
  .inbox-grid:not(.has-mobile-reader) .reader-pane, .inbox-grid.has-mobile-reader .stream-pane { display: none; }
}
@media (max-width: 640px) {
  .topbar { grid-template-columns: minmax(0, 1fr) auto auto; gap: 10px; }
  .language-controls { grid-column: 2; grid-row: 1; }
  .window-controls { grid-column: 3; grid-row: 1; }
  .search-wrap { order: 0; grid-column: 1 / -1; grid-row: 2; }
  .mailbox-controls { grid-column: 1 / -1; grid-row: 3; justify-self: stretch; }
  .acct-switcher { flex: 1; min-width: 0; }
  .topbar .acct-trigger { width: 100%; max-width: none; justify-content: flex-start; }
  .acct-chevron, .topbar .btn-primary, .filter-bar-right .sort-btn { margin-left: auto; }
  .acct-menu { left: 0; right: auto; }
  .filter-bar { flex-wrap: wrap; gap: 8px; }
  .category-pills { flex-basis: 100%; }
  .filter-bar-right { width: 100%; justify-content: space-between; }
  .filter-bar-right .sort-btn { min-height: 40px; }
}
@supports not (backdrop-filter: blur(1px)) { .glass-pill { background: var(--bg-surface); border-color: var(--border-subtle); } }
@media (prefers-reduced-transparency: reduce) { .glass-pill { background: var(--bg-surface); -webkit-backdrop-filter: none; backdrop-filter: none; } }
@media (prefers-contrast: more) { .glass-pill { background: var(--bg-surface); border-color: var(--text-secondary); } .cat-count { color: var(--text-secondary); } }
@media (prefers-reduced-motion: reduce) { *, *::before, *::after { transition-duration: 0.01ms !important; animation-duration: 0.01ms !important; } }
"##;

pub const INDEX_HTML: &str = r##"<!doctype html>
<html lang="vi">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Vortex</title>
  <link rel="icon" type="image/svg+xml" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100' fill='%23ffffff'%3E%3Cpath d='M74.6 8.6L69.4 6.9L62.9 6.0L57.2 6.3L49.8 8.0L44.5 10.2L41.0 12.1L34.8 16.8L30.5 21.4L27.7 25.4L24.7 31.8L23.2 37.2L22.6 44.0L23.2 49.1L25.0 55.0L27.0 59.0L30.6 64.0L25.8 60.8L23.3 58.4L19.9 54.3L16.8 49.0L14.3 42.2L13.4 37.1L13.4 29.4L13.8 27.5L15.3 22.2L16.7 19.3L19.1 15.7L24.2 10.2L30.1 6.1L35.8 3.2L39.5 2.0L44.6 0.9L52.0 0.6L55.4 0.9L62.0 2.3L68.3 4.6L72.2 6.7ZM91.4 74.3L93.4 67.7L94.0 63.7L94.0 58.3L93.7 56.0L91.7 48.9L88.8 43.0L86.1 39.1L81.1 33.8L76.8 30.4L73.4 28.3L68.9 26.0L63.9 24.1L60.5 23.2L56.6 22.6L50.0 22.6L43.0 24.1L38.0 26.3L33.9 29.1L30.3 32.6L31.2 30.4L33.7 26.8L39.0 21.3L44.6 17.4L48.4 15.5L53.5 13.8L59.4 12.6L66.6 12.6L68.8 12.9L75.6 15.0L82.2 18.7L85.1 20.9L89.6 25.5L93.3 30.7L96.8 38.1L98.5 45.8L98.8 53.7L97.7 60.8L95.2 67.8ZM12.0 21.7L12.2 22.2L10.1 26.7L9.5 28.9L8.6 32.6L8.3 35.7L8.6 41.7L10.1 47.6L12.1 52.5L15.9 58.6L20.9 64.2L25.5 67.9L28.1 69.6L32.5 71.8L38.0 73.7L44.9 74.6L51.1 74.2L57.3 72.8L62.8 70.2L65.9 67.9L70.1 63.8L71.4 61.7L71.6 62.2L70.8 64.2L68.2 69.0L66.2 71.6L62.5 75.4L57.4 78.9L53.6 80.8L48.5 82.8L43.4 84.0L38.6 84.6L31.2 84.2L25.8 83.1L22.1 81.9L15.8 78.5L13.2 76.5L9.7 72.9L6.4 68.5L4.9 65.7L2.3 58.8L1.2 51.1L1.5 44.6L3.2 37.0L6.7 29.2ZM15.1 82.9L21.0 86.4L24.7 87.9L29.8 89.4L35.4 90.3L44.3 90.0L53.0 87.9L59.0 85.3L63.8 82.3L69.4 77.4L73.3 71.9L75.3 67.9L76.8 63.6L77.4 60.5L77.7 53.2L76.8 47.5L74.3 41.1L72.5 38.1L69.2 33.9L65.1 30.6L67.0 31.0L71.6 33.5L76.6 37.4L79.2 39.9L83.3 45.2L85.1 48.3L87.6 53.5L89.0 58.1L90.0 64.6L90.0 67.1L89.0 72.5L87.9 75.9L85.9 79.9L83.8 83.2L81.1 86.5L75.6 91.3L71.6 93.9L67.6 95.9L63.0 97.6L58.2 98.8L49.4 99.7L42.9 99.1L40.3 98.5L34.3 96.8L30.5 95.2L26.4 93.0L21.4 89.5L16.9 85.4ZM30.9 50.3a18.7 18.7 0 1 0 37.4 0a18.7 18.7 0 1 0 -37.4 0Z'/%3E%3C/svg%3E">
  <link rel="stylesheet" href="/tokens.css">
</head>
<body>
  <!-- Official Heroicons v2 20/solid SVG sprite (fill="currentColor") -->
  <svg class="svg-sprite" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
    <symbol id="vortex-logo" viewBox="0 0 100 100" fill="currentColor">
      <path d="M74.6 8.6L69.4 6.9L62.9 6.0L57.2 6.3L49.8 8.0L44.5 10.2L41.0 12.1L34.8 16.8L30.5 21.4L27.7 25.4L24.7 31.8L23.2 37.2L22.6 44.0L23.2 49.1L25.0 55.0L27.0 59.0L30.6 64.0L25.8 60.8L23.3 58.4L19.9 54.3L16.8 49.0L14.3 42.2L13.4 37.1L13.4 29.4L13.8 27.5L15.3 22.2L16.7 19.3L19.1 15.7L24.2 10.2L30.1 6.1L35.8 3.2L39.5 2.0L44.6 0.9L52.0 0.6L55.4 0.9L62.0 2.3L68.3 4.6L72.2 6.7ZM91.4 74.3L93.4 67.7L94.0 63.7L94.0 58.3L93.7 56.0L91.7 48.9L88.8 43.0L86.1 39.1L81.1 33.8L76.8 30.4L73.4 28.3L68.9 26.0L63.9 24.1L60.5 23.2L56.6 22.6L50.0 22.6L43.0 24.1L38.0 26.3L33.9 29.1L30.3 32.6L31.2 30.4L33.7 26.8L39.0 21.3L44.6 17.4L48.4 15.5L53.5 13.8L59.4 12.6L66.6 12.6L68.8 12.9L75.6 15.0L82.2 18.7L85.1 20.9L89.6 25.5L93.3 30.7L96.8 38.1L98.5 45.8L98.8 53.7L97.7 60.8L95.2 67.8ZM12.0 21.7L12.2 22.2L10.1 26.7L9.5 28.9L8.6 32.6L8.3 35.7L8.6 41.7L10.1 47.6L12.1 52.5L15.9 58.6L20.9 64.2L25.5 67.9L28.1 69.6L32.5 71.8L38.0 73.7L44.9 74.6L51.1 74.2L57.3 72.8L62.8 70.2L65.9 67.9L70.1 63.8L71.4 61.7L71.6 62.2L70.8 64.2L68.2 69.0L66.2 71.6L62.5 75.4L57.4 78.9L53.6 80.8L48.5 82.8L43.4 84.0L38.6 84.6L31.2 84.2L25.8 83.1L22.1 81.9L15.8 78.5L13.2 76.5L9.7 72.9L6.4 68.5L4.9 65.7L2.3 58.8L1.2 51.1L1.5 44.6L3.2 37.0L6.7 29.2ZM15.1 82.9L21.0 86.4L24.7 87.9L29.8 89.4L35.4 90.3L44.3 90.0L53.0 87.9L59.0 85.3L63.8 82.3L69.4 77.4L73.3 71.9L75.3 67.9L76.8 63.6L77.4 60.5L77.7 53.2L76.8 47.5L74.3 41.1L72.5 38.1L69.2 33.9L65.1 30.6L67.0 31.0L71.6 33.5L76.6 37.4L79.2 39.9L83.3 45.2L85.1 48.3L87.6 53.5L89.0 58.1L90.0 64.6L90.0 67.1L89.0 72.5L87.9 75.9L85.9 79.9L83.8 83.2L81.1 86.5L75.6 91.3L71.6 93.9L67.6 95.9L63.0 97.6L58.2 98.8L49.4 99.7L42.9 99.1L40.3 98.5L34.3 96.8L30.5 95.2L26.4 93.0L21.4 89.5L16.9 85.4ZM30.9 50.3a18.7 18.7 0 1 0 37.4 0a18.7 18.7 0 1 0 -37.4 0Z"/>
    </symbol>
    <symbol id="hi-inbox" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M1 11.27c0-.246.033-.492.099-.73l1.523-5.521A2.75 2.75 0 0 1 5.273 3h9.454a2.75 2.75 0 0 1 2.651 2.019l1.523 5.52c.066.239.099.485.099.732V15a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2v-3.73Zm3.068-5.852A1.25 1.25 0 0 1 5.273 4.5h9.454a1.25 1.25 0 0 1 1.205.918l1.523 5.52c.006.02.01.041.015.062H14a1 1 0 0 0-.86.49l-.606 1.02a1 1 0 0 1-.86.49H8.236a1 1 0 0 1-.894-.553l-.448-.894A1 1 0 0 0 6 11H2.53l.015-.062 1.523-5.52Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-key" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M8 7a5 5 0 1 1 3.61 4.804l-1.903 1.903A1 1 0 0 1 9 14H8v1a1 1 0 0 1-1 1H6v1a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1v-2a1 1 0 0 1 .293-.707L8.196 8.39A5.002 5.002 0 0 1 8 7Zm5-3a.75.75 0 0 0 0 1.5A1.5 1.5 0 0 1 14.5 7 .75.75 0 0 0 16 7a3 3 0 0 0-3-3Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-credit-card" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M2.5 4A1.5 1.5 0 0 0 1 5.5V6h18v-.5A1.5 1.5 0 0 0 17.5 4h-15ZM19 8.5H1v6A1.5 1.5 0 0 0 2.5 16h15a1.5 1.5 0 0 0 1.5-1.5v-6ZM3 13.25a.75.75 0 0 1 .75-.75h1.5a.75.75 0 0 1 0 1.5h-1.5a.75.75 0 0 1-.75-.75Zm4.75-.75a.75.75 0 0 0 0 1.5h3.5a.75.75 0 0 0 0-1.5h-3.5Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-truck" viewBox="0 0 20 20" fill="currentColor">
      <path d="M6.5 3c-1.051 0-2.093.04-3.125.117A1.49 1.49 0 0 0 2 4.607V10.5h9V4.606c0-.771-.59-1.43-1.375-1.489A41.568 41.568 0 0 0 6.5 3ZM2 12v2.5A1.5 1.5 0 0 0 3.5 16h.041a3 3 0 0 1 5.918 0h.791a.75.75 0 0 0 .75-.75V12H2Z"/>
      <path d="M6.5 18a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3ZM13.25 5a.75.75 0 0 0-.75.75v8.514a3.001 3.001 0 0 1 4.893 1.44c.37-.275.61-.719.595-1.227a24.905 24.905 0 0 0-1.784-8.549A1.486 1.486 0 0 0 14.823 5H13.25ZM14.5 18a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3Z"/>
    </symbol>
    <symbol id="hi-shield-check" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M9.661 2.237a.531.531 0 0 1 .678 0 11.947 11.947 0 0 0 7.078 2.749.5.5 0 0 1 .479.425c.069.52.104 1.05.104 1.59 0 5.162-3.26 9.563-7.834 11.256a.48.48 0 0 1-.332 0C5.26 16.564 2 12.163 2 7c0-.538.035-1.069.104-1.589a.5.5 0 0 1 .48-.425 11.947 11.947 0 0 0 7.077-2.75Zm4.196 5.954a.75.75 0 0 0-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 1 0-1.06 1.061l2.5 2.5a.75.75 0 0 0 1.137-.089l4-5.5Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-shield-exclamation" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M10.339 2.237a.531.531 0 0 0-.678 0 11.947 11.947 0 0 1-7.078 2.75.5.5 0 0 0-.479.425A12.11 12.11 0 0 0 2 7c0 5.163 3.26 9.564 7.834 11.257a.48.48 0 0 0 .332 0C14.74 16.564 18 12.163 18 7c0-.538-.035-1.069-.104-1.589a.5.5 0 0 0-.48-.425 11.947 11.947 0 0 1-7.077-2.75ZM10 6a.75.75 0 0 1 .75.75v3.5a.75.75 0 0 1-1.5 0v-3.5A.75.75 0 0 1 10 6Zm0 9a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-briefcase" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M6 3.75A2.75 2.75 0 0 1 8.75 1h2.5A2.75 2.75 0 0 1 14 3.75v.443c.572.055 1.14.122 1.706.2C17.053 4.582 18 5.75 18 7.07v3.469c0 1.126-.694 2.191-1.83 2.54-1.952.599-4.024.921-6.17.921s-4.219-.322-6.17-.921C2.694 12.73 2 11.665 2 10.539V7.07c0-1.321.947-2.489 2.294-2.676A41.047 41.047 0 0 1 6 4.193V3.75Zm6.5 0v.325a41.622 41.622 0 0 0-5 0V3.75c0-.69.56-1.25 1.25-1.25h2.5c.69 0 1.25.56 1.25 1.25ZM10 10a1 1 0 0 0-1 1v.01a1 1 0 0 0 1 1h.01a1 1 0 0 0 1-1V11a1 1 0 0 0-1-1H10Z" clip-rule="evenodd"/>
      <path d="M3 15.055v-.684c.126.053.255.1.39.142 2.092.642 4.313.987 6.61.987 2.297 0 4.518-.345 6.61-.987.135-.041.264-.089.39-.142v.684c0 1.347-.985 2.53-2.363 2.686a41.454 41.454 0 0 1-9.274 0C3.985 17.585 3 16.402 3 15.055Z"/>
    </symbol>
    <symbol id="hi-user-group" viewBox="0 0 20 20" fill="currentColor">
      <path d="M10 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6ZM6 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0ZM1.49 15.326a.78.78 0 0 1-.358-.442 3 3 0 0 1 4.308-3.516 6.484 6.484 0 0 0-1.905 3.959c-.023.222-.014.442.025.654a4.97 4.97 0 0 1-2.07-.655ZM16.44 15.98a4.97 4.97 0 0 0 2.07-.654.78.78 0 0 0 .357-.442 3 3 0 0 0-4.308-3.517 6.484 6.484 0 0 1 1.907 3.96 2.32 2.32 0 0 1-.026.654ZM18 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0ZM5.304 16.19a.844.844 0 0 1-.277-.71 5 5 0 0 1 9.947 0 .843.843 0 0 1-.277.71A6.975 6.975 0 0 1 10 18a6.974 6.974 0 0 1-4.696-1.81Z"/>
    </symbol>
    <symbol id="hi-tag" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M4.5 2A2.5 2.5 0 0 0 2 4.5v3.879a2.5 2.5 0 0 0 .732 1.767l7.5 7.5a2.5 2.5 0 0 0 3.536 0l3.878-3.878a2.5 2.5 0 0 0 0-3.536l-7.5-7.5A2.5 2.5 0 0 0 8.38 2H4.5ZM5 6a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-arrow-path" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M15.312 11.424a5.5 5.5 0 0 1-9.201 2.466l-.312-.311h2.433a.75.75 0 0 0 0-1.5H3.989a.75.75 0 0 0-.75.75v4.242a.75.75 0 0 0 1.5 0v-2.43l.31.31a7 7 0 0 0 11.712-3.138.75.75 0 0 0-1.449-.39Zm1.23-3.723a.75.75 0 0 0 .219-.53V2.929a.75.75 0 0 0-1.5 0V5.36l-.31-.31A7 7 0 0 0 3.239 8.188a.75.75 0 1 0 1.448.389A5.5 5.5 0 0 1 13.89 6.11l.311.31h-2.432a.75.75 0 0 0 0 1.5h4.243a.75.75 0 0 0 .53-.219Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-user-plus" viewBox="0 0 20 20" fill="currentColor">
      <path d="M10 5a3 3 0 1 1-6 0 3 3 0 0 1 6 0ZM1.615 16.428a1.224 1.224 0 0 1-.569-1.175 6.002 6.002 0 0 1 11.908 0c.058.467-.172.92-.57 1.174A9.953 9.953 0 0 1 7 18a9.953 9.953 0 0 1-5.385-1.572ZM16.25 5.75a.75.75 0 0 0-1.5 0v2h-2a.75.75 0 0 0 0 1.5h2v2a.75.75 0 0 0 1.5 0v-2h2a.75.75 0 0 0 0-1.5h-2v-2Z"/>
    </symbol>
    <symbol id="hi-language" viewBox="0 0 20 20" fill="currentColor">
      <path d="M7.75 2.75a.75.75 0 0 0-1.5 0v1.258a32.987 32.987 0 0 0-3.599.278.75.75 0 1 0 .198 1.487A31.545 31.545 0 0 1 8.7 5.545 19.381 19.381 0 0 1 7 9.56a19.418 19.418 0 0 1-1.002-2.05.75.75 0 0 0-1.384.577 20.935 20.935 0 0 0 1.492 2.91 19.613 19.613 0 0 1-3.828 4.154.75.75 0 1 0 .945 1.164A21.116 21.116 0 0 0 7 12.331c.095.132.192.262.29.391a.75.75 0 0 0 1.194-.91c-.204-.266-.4-.538-.59-.815a20.888 20.888 0 0 0 2.333-5.332c.31.031.618.068.924.108a.75.75 0 0 0 .198-1.487 32.832 32.832 0 0 0-3.599-.278V2.75Z"/>
      <path fill-rule="evenodd" d="M13 8a.75.75 0 0 1 .671.415l4.25 8.5a.75.75 0 1 1-1.342.67L15.787 16h-5.573l-.793 1.585a.75.75 0 1 1-1.342-.67l4.25-8.5A.75.75 0 0 1 13 8Zm2.037 6.5L13 10.427 10.964 14.5h4.073Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-magnifying-glass" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11ZM2 9a7 7 0 1 1 12.452 4.391l3.328 3.329a.75.75 0 1 1-1.06 1.06l-3.329-3.328A7 7 0 0 1 2 9Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-clipboard-document" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M15.988 3.012A2.25 2.25 0 0 1 18 5.25v6.5A2.25 2.25 0 0 1 15.75 14H13.5v-3.379a3 3 0 0 0-.879-2.121l-3.12-3.121a3 3 0 0 0-1.402-.791 2.252 2.252 0 0 1 1.913-1.576A2.25 2.25 0 0 1 12.25 1h1.5a2.25 2.25 0 0 1 2.238 2.012ZM11.5 3.25a.75.75 0 0 1 .75-.75h1.5a.75.75 0 0 1 .75.75v.25h-3v-.25Z" clip-rule="evenodd"/>
      <path d="M3.5 6A1.5 1.5 0 0 0 2 7.5v9A1.5 1.5 0 0 0 3.5 18h7a1.5 1.5 0 0 0 1.5-1.5v-5.879a1.5 1.5 0 0 0-.44-1.06L8.44 6.439A1.5 1.5 0 0 0 7.378 6H3.5Z"/>
    </symbol>
    <symbol id="hi-archive-box" viewBox="0 0 20 20" fill="currentColor">
      <path d="M2 3a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1H2Z"/>
      <path fill-rule="evenodd" d="M2 7.5h16l-.811 7.71a2 2 0 0 1-1.99 1.79H4.802a2 2 0 0 1-1.99-1.79L2 7.5ZM7 11a1 1 0 0 1 1-1h4a1 1 0 1 1 0 2H8a1 1 0 0 1-1-1Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-envelope-open" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M2.106 6.447A2 2 0 0 0 1 8.237V16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V8.236a2 2 0 0 0-1.106-1.789l-7-3.5a2 2 0 0 0-1.788 0l-7 3.5Zm1.48 4.007a.75.75 0 0 0-.671 1.342l5.855 2.928a2.75 2.75 0 0 0 2.46 0l5.852-2.927a.75.75 0 1 0-.67-1.341l-5.853 2.926a1.25 1.25 0 0 1-1.118 0l-5.856-2.928Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-bookmark" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M10 2c-1.716 0-3.408.106-5.07.31C3.806 2.45 3 3.414 3 4.517V17.25a.75.75 0 0 0 1.075.676L10 15.082l5.925 2.844A.75.75 0 0 0 17 17.25V4.517c0-1.103-.806-2.068-1.93-2.207A41.403 41.403 0 0 0 10 2Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-trash" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M8.75 1A2.75 2.75 0 0 0 6 3.75v.443c-.795.077-1.584.176-2.365.298a.75.75 0 1 0 .23 1.482l.149-.022.841 10.518A2.75 2.75 0 0 0 7.596 19h4.807a2.75 2.75 0 0 0 2.742-2.53l.841-10.52.149.023a.75.75 0 0 0 .23-1.482A41.03 41.03 0 0 0 14 4.193V3.75A2.75 2.75 0 0 0 11.25 1h-2.5ZM10 4c.84 0 1.673.025 2.5.075V3.75c0-.69-.56-1.25-1.25-1.25h-2.5c-.69 0-1.25.56-1.25 1.25v.325C8.327 4.025 9.16 4 10 4ZM8.58 7.72a.75.75 0 0 0-1.5.06l.3 7.5a.75.75 0 1 0 1.5-.06l-.3-7.5Zm4.34.06a.75.75 0 1 0-1.5-.06l-.3 7.5a.75.75 0 1 0 1.5.06l.3-7.5Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-check-circle" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16Zm3.857-9.809a.75.75 0 0 0-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 1 0-1.06 1.061l2.5 2.5a.75.75 0 0 0 1.137-.089l4-5.5Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-x-mark" viewBox="0 0 20 20" fill="currentColor">
      <path d="M6.28 5.22a.75.75 0 0 0-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 1 0 1.06 1.06L10 11.06l3.72 3.72a.75.75 0 1 0 1.06-1.06L11.06 10l3.72-3.72a.75.75 0 0 0-1.06-1.06L10 8.94 6.28 5.22Z"/>
    </symbol>
    <symbol id="hi-user-circle" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M18 10a8 8 0 1 1-16 0 8 8 0 0 1 16 0Zm-5.5-2.5a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0ZM10 12a5.99 5.99 0 0 0-4.793 2.39A6.483 6.483 0 0 0 10 16.5a6.483 6.483 0 0 0 4.793-2.11A5.99 5.99 0 0 0 10 12Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-arrows-up-down" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M2.24 6.8a.75.75 0 0 0 1.06-.04l1.95-2.1v8.59a.75.75 0 0 0 1.5 0V4.66l1.95 2.1a.75.75 0 1 0 1.1-1.02l-3.25-3.5a.75.75 0 0 0-1.1 0L2.2 5.74a.75.75 0 0 0 .04 1.06Zm8 6.4a.75.75 0 0 0-.04 1.06l3.25 3.5a.75.75 0 0 0 1.1 0l3.25-3.5a.75.75 0 1 0-1.1-1.02l-1.95 2.1V6.75a.75.75 0 0 0-1.5 0v8.59l-1.95-2.1a.75.75 0 0 0-1.06-.04Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-sun" viewBox="0 0 20 20" fill="currentColor">
      <path d="M10 2a.75.75 0 0 1 .75.75v1.5a.75.75 0 0 1-1.5 0v-1.5A.75.75 0 0 1 10 2ZM10 15a.75.75 0 0 1 .75.75v1.5a.75.75 0 0 1-1.5 0v-1.5A.75.75 0 0 1 10 15ZM10 7a3 3 0 1 0 0 6 3 3 0 0 0 0-6ZM15.657 5.404a.75.75 0 1 0-1.06-1.06l-1.061 1.06a.75.75 0 0 0 1.06 1.06l1.06-1.06ZM6.464 14.596a.75.75 0 1 0-1.06-1.06l-1.06 1.06a.75.75 0 0 0 1.06 1.06l1.06-1.06ZM18 10a.75.75 0 0 1-.75.75h-1.5a.75.75 0 0 1 0-1.5h1.5A.75.75 0 0 1 18 10ZM5 10a.75.75 0 0 1-.75.75h-1.5a.75.75 0 0 1 0-1.5h1.5A.75.75 0 0 1 5 10ZM14.596 15.657a.75.75 0 0 0 1.06-1.06l-1.06-1.061a.75.75 0 1 0-1.06 1.06l1.06 1.06ZM5.404 6.464a.75.75 0 0 0 1.06-1.06l-1.06-1.06a.75.75 0 1 0-1.061 1.06l1.06 1.06Z"/>
    </symbol>
    <symbol id="hi-moon" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M7.455 2.004a.75.75 0 0 1 .26.77 7 7 0 0 0 9.958 7.967.75.75 0 0 1 1.067.853A8.5 8.5 0 1 1 6.647 1.921a.75.75 0 0 1 .808.083Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-speaker-wave" viewBox="0 0 20 20" fill="currentColor">
      <path d="M10.5 3.75a.75.75 0 0 0-1.264-.546L5.203 7H2.667a.75.75 0 0 0-.7.48A6.985 6.985 0 0 0 1.5 10c0 .887.165 1.737.468 2.52.111.29.39.48.7.48h2.535l4.033 3.796a.75.75 0 0 0 1.264-.546V3.75ZM16.45 5.05a.75.75 0 0 0-1.06 1.061 5.5 5.5 0 0 1 0 7.778.75.75 0 0 0 1.06 1.06 7 7 0 0 0 0-9.899Z"/>
      <path d="M14.329 7.172a.75.75 0 0 0-1.061 1.06 2.5 2.5 0 0 1 0 3.536.75.75 0 0 0 1.06 1.06 4 4 0 0 0 0-5.656Z"/>
    </symbol>
    <symbol id="hi-speaker-x-mark" viewBox="0 0 20 20" fill="currentColor">
      <path d="M10.5 3.75a.75.75 0 0 0-1.264-.546L5.203 7H2.667a.75.75 0 0 0-.7.48A6.985 6.985 0 0 0 1.5 10c0 .887.165 1.737.468 2.52.111.29.39.48.7.48h2.535l4.033 3.796a.75.75 0 0 0 1.264-.546V3.75ZM13.78 7.22a.75.75 0 1 0-1.06 1.06L14.44 10l-1.72 1.72a.75.75 0 1 0 1.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 1 0 1.06-1.06L16.56 10l1.72-1.72a.75.75 0 0 0-1.06-1.06L15.5 8.94l-1.72-1.72Z"/>
    </symbol>
    <symbol id="hi-minus" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M4 10a.75.75 0 0 1 .75-.75h10.5a.75.75 0 0 1 0 1.5H4.75A.75.75 0 0 1 4 10Z" clip-rule="evenodd"/>
    </symbol>
    <symbol id="hi-arrows-pointing-out" viewBox="0 0 20 20" fill="currentColor">
      <path d="M13.28 7.78l3.22-3.22v2.69a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.69l-3.22 3.22a.75.75 0 0 0 1.06 1.06ZM2 17.25v-4.5a.75.75 0 0 1 1.5 0v2.69l3.22-3.22a.75.75 0 0 1 1.06 1.06L4.56 16.5h2.69a.75.75 0 0 1 0 1.5h-4.5a.747.747 0 0 1-.75-.75ZM12.22 13.28l3.22 3.22h-2.69a.75.75 0 0 0 0 1.5h4.5a.747.747 0 0 0 .75-.75v-4.5a.75.75 0 0 0-1.5 0v2.69l-3.22-3.22a.75.75 0 1 0-1.06 1.06ZM3.5 4.56l3.22 3.22a.75.75 0 0 0 1.06-1.06L4.56 3.5h2.69a.75.75 0 0 0 0-1.5h-4.5a.75.75 0 0 0-.75.75v4.5a.75.75 0 0 0 1.5 0V4.56Z"/>
    </symbol>
  </svg>

  <div class="shell">
    <header class="topbar">
      <span class="brand-mark">
        <svg class="hi-icon-lg" aria-hidden="true"><use href="#vortex-logo"></use></svg>
        <h1 class="brand-title">Vortex</h1>
      </span>

      <div id="topSearchWrap" class="search-wrap glass-pill" role="search">
        <svg class="hi-icon" aria-hidden="true"><use href="#hi-magnifying-glass"></use></svg>
        <input id="searchInput" type="search" class="input-field" aria-label="Tìm kiếm thư, người gửi hoặc mã xác nhận" placeholder="Tìm kiếm thư, người gửi hoặc mã xác nhận...">
        <button id="btnSync" type="button" class="btn refresh-btn" aria-label="Làm mới" title="Làm mới">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-arrow-path"></use></svg>
        </button>
      </div>

      <div class="mailbox-controls glass-pill" role="group" aria-label="Quản lý hộp thư">
        <div id="acctSwitcher" class="acct-switcher hidden">
          <button id="btnAcctMenu" type="button" class="btn acct-trigger" aria-expanded="false" aria-controls="acctMenu" aria-haspopup="true">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-user-circle"></use></svg>
            <span id="acctActiveLabel" class="acct-active-label">Tất cả hộp thư</span>
            <svg class="acct-chevron" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true"><path fill-rule="evenodd" d="M5.22 7.22a.75.75 0 0 1 1.06 0L10 10.94l3.72-3.72a.75.75 0 0 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 8.28a.75.75 0 0 1 0-1.06Z" clip-rule="evenodd"/></svg>
          </button>
          <div id="acctMenu" class="acct-menu hidden" role="menu"></div>
        </div>
        <button id="btnToggleSetup" type="button" class="btn btn-primary">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-user-plus"></use></svg>
          <span id="btnToggleSetupLabel" data-i18n="connect_account">Thêm hộp thư</span>
        </button>
      </div>
      <div class="language-controls glass-pill">
        <button id="btnSfx" type="button" class="btn btn-ghost" aria-label="Âm thanh: Bật" title="Âm thanh: Bật">
          <svg class="hi-icon" aria-hidden="true"><use id="sfxIconUse" href="#hi-speaker-wave"></use></svg>
        </button>
        <button id="btnTheme" type="button" class="btn btn-ghost" aria-label="Đổi giao diện Sáng/Tối">
          <svg class="hi-icon" aria-hidden="true"><use id="themeIconUse" href="#hi-moon"></use></svg>
          <span id="themeLabel">Tối</span>
        </button>
        <button id="btnLang" type="button" class="btn btn-ghost" aria-label="Switch to English">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-language"></use></svg>
          <span id="langLabel">EN</span>
        </button>
      </div>
      <div class="window-controls glass-pill" role="group" aria-label="Cửa sổ">
        <button id="btnWinMin" type="button" class="btn btn-ghost win-btn" aria-label="Thu nhỏ" title="Thu nhỏ">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-minus"></use></svg>
        </button>
        <button id="btnWinMax" type="button" class="btn btn-ghost win-btn" aria-label="Phóng to" title="Phóng to">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-arrows-pointing-out"></use></svg>
        </button>
        <button id="btnWinClose" type="button" class="btn btn-ghost win-btn win-btn-close" aria-label="Đóng" title="Đóng">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-x-mark"></use></svg>
        </button>
      </div>
    </header>

    <!-- SCREEN 1: Account Management & Multi-Account Onboarding View -->
    <section id="onboardingView" class="onboarding-wrap hidden" aria-label="Thêm hộp thư">
      <div class="onboarding-head">
        <div>
          <h2 class="onboarding-title" data-i18n="setup_heading">Thêm hộp thư</h2>
          <p class="onboarding-sub" data-i18n="setup_note">Kết nối một hoặc nhiều hộp thư để tự động lấy mã xác nhận (OTP) cùng lúc.</p>
        </div>
        <button id="btnCloseSetup" type="button" class="btn btn-ghost hidden">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-x-mark"></use></svg>
          <span data-i18n="back_to_inbox">Quay lại hộp thư</span>
        </button>
      </div>

      <div class="mode-switch" role="group" aria-label="Chế độ thêm tài khoản">
        <button id="btnModeSingle" type="button" class="preset-btn" aria-pressed="true">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-user-plus"></use></svg>
          <span data-i18n="mode_single">Thêm 1 tài khoản</span>
        </button>
        <button id="btnModeBulk" type="button" class="preset-btn" aria-pressed="false">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-user-group"></use></svg>
          <span data-i18n="mode_bulk">Thêm nhanh nhiều tài khoản</span>
        </button>
      </div>

      <div id="singleAddWrap">
        <button id="btnGoogleLogin" type="button" class="btn btn-google">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-shield-check"></use></svg>
          <span id="btnGoogleLoginText" data-i18n="google_login_btn">Đăng nhập nhanh bằng Google</span>
        </button>

        <p class="divider-caption" data-i18n="manual_divider">Hoặc chọn dịch vụ để nhập mật khẩu ứng dụng thủ công:</p>

        <div class="provider-grid" role="group" aria-label="Chọn dịch vụ">
          <button type="button" class="preset-btn" data-preset="gmail" aria-pressed="true">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-inbox"></use></svg>
            <span>Gmail</span>
          </button>
          <button type="button" class="preset-btn" data-preset="outlook" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-briefcase"></use></svg>
            <span>Outlook</span>
          </button>
          <button type="button" class="preset-btn" data-preset="icloud" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-shield-check"></use></svg>
            <span>iCloud</span>
          </button>
          <button type="button" class="preset-btn" data-preset="custom_imap" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-key"></use></svg>
            <span data-i18n="preset_custom">Email khác</span>
          </button>
        </div>

        <form id="accountForm" class="form-stack">
          <input id="accPreset" type="hidden" value="gmail">
          <div>
            <label id="lblEmail" class="field-label" for="accEmail" data-i18n="email_label">Địa chỉ Email</label>
            <input id="accEmail" class="input-field" type="text" required placeholder="tenban@gmail.com">
          </div>

          <div>
            <label id="lblSecret" class="field-label" for="accSecret" data-i18n="secret_label">Mật khẩu ứng dụng</label>
            <div class="secret-input-group">
              <input id="accSecret" class="input-field" type="password" required placeholder="Nhập mật khẩu ứng dụng">
              <button id="btnTogglePw" type="button" class="btn" data-i18n="show_pw">Hiện</button>
            </div>
            <p id="providerGuide" class="field-hint">Mẹo: Với Gmail, dùng Mật khẩu ứng dụng 16 ký tự trong phần Bảo mật của tài khoản Google.</p>
          </div>

          <div id="customServerWrap" class="server-grid hidden">
            <div>
              <label class="field-label" for="accHost" data-i18n="host_label">Máy chủ</label>
              <input id="accHost" class="input-field" type="text" required value="imap.gmail.com">
            </div>
            <div>
              <label class="field-label" for="accPort" data-i18n="port_label">Cổng</label>
              <input id="accPort" class="input-field" type="number" required value="993">
            </div>
          </div>

          <button id="btnSubmitAccount" type="submit" class="btn btn-primary btn-large">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-check-circle"></use></svg>
            <span id="btnSubmitAccountText" data-i18n="save_connect">Kết nối hộp thư</span>
          </button>
        </form>
      </div>

      <form id="bulkAccountForm" class="form-stack hidden">
        <div>
          <label class="field-label" for="bulkAccountsInput" data-i18n="bulk_label">Danh sách tài khoản (mỗi dòng 1 tài khoản)</label>
          <textarea id="bulkAccountsInput" class="input-field bulk-textarea" placeholder="acc1@gmail.com|abcd efgh ijkl mnop&#10;acc2@outlook.com|mat_khau_ung_dung&#10;acc3@domain.com|mat_khau|imap.domain.com|993"></textarea>
          <p class="field-hint" data-i18n="bulk_hint">Định dạng: email|mật_khẩu_ứng_dụng hoặc email:mật_khẩu. Hệ thống tự nhận diện máy chủ Gmail, Outlook, Hotmail, iCloud, Yahoo và đồng bộ song song tất cả tài khoản.</p>
        </div>
        <button id="btnSubmitBulk" type="submit" class="btn btn-primary btn-large">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-check-circle"></use></svg>
          <span id="btnSubmitBulkText" data-i18n="bulk_submit">Kết nối và lấy mã tất cả</span>
        </button>
      </form>

      <div id="setupErrorBox" class="phishing-box hidden">
        <svg class="hi-icon-lg" aria-hidden="true"><use href="#hi-shield-exclamation"></use></svg>
        <span id="setupErrorText"></span>
      </div>
    </section>

    <!-- SCREEN 2: Clean 2-Column Inbox View -->
    <main id="inboxView" aria-label="Hộp thư">
      <div class="filter-bar">
        <nav id="categoryNav" class="category-pills glass-pill" aria-label="Bộ lọc thư">
          <span id="catGlider" class="cat-glider" aria-hidden="true"></span>
          <button type="button" class="cat-btn" data-cat="" aria-pressed="true">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-inbox"></use></svg>
            <span data-i18n="cat_all">Tất cả</span>
            <span id="cnt_all" class="cat-count"></span>
          </button>
          <button type="button" class="cat-btn" data-cat="code_otp" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-key"></use></svg>
            <span data-i18n="cat_otp">Mã OTP</span>
            <span id="cnt_code_otp" class="cat-count"></span>
          </button>
          <button type="button" class="cat-btn" data-cat="billing" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-credit-card"></use></svg>
            <span data-i18n="cat_billing">Hóa đơn</span>
            <span id="cnt_billing" class="cat-count"></span>
          </button>
          <button type="button" class="cat-btn" data-cat="shipping" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-truck"></use></svg>
            <span data-i18n="cat_shipping">Đơn hàng</span>
            <span id="cnt_shipping" class="cat-count"></span>
          </button>
          <button type="button" class="cat-btn" data-cat="security_alert" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-shield-check"></use></svg>
            <span data-i18n="cat_security">Bảo mật</span>
            <span id="cnt_security_alert" class="cat-count"></span>
          </button>
          <button type="button" class="cat-btn" data-cat="work" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-briefcase"></use></svg>
            <span data-i18n="cat_work">Công việc</span>
            <span id="cnt_work" class="cat-count"></span>
          </button>
          <button type="button" class="cat-btn" data-cat="promo_newsletter" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-tag"></use></svg>
            <span data-i18n="cat_promo">Khuyến mãi</span>
            <span id="cnt_promo_newsletter" class="cat-count"></span>
          </button>
        </nav>
        <div class="filter-bar-right">
          <span id="statusNotice" class="otp-meta" role="status"></span>
          <button id="btnAutoSort" type="button" class="btn btn-sm sort-btn glass-pill" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-arrows-up-down"></use></svg>
            <span id="btnAutoSortLabel" data-i18n="auto_sort">Tự động sắp xếp</span>
          </button>
        </div>
      </div>

      <div id="inboxGrid" class="inbox-grid">
        <section class="stream-pane" aria-label="Danh sách thư">
          <ul id="streamList" class="stream-list"></ul>
        </section>

        <section class="reader-pane" aria-label="Nội dung thư">
          <div id="readerEmptyState" class="reader-empty">
            <svg class="hi-icon-lg" aria-hidden="true"><use href="#hi-envelope-open"></use></svg>
            <p style="margin-top:8px;" data-i18n="select_item_prompt">Chọn một thư ở bên trái để đọc nội dung</p>
          </div>

          <div id="readerContentWrap" class="reader-content-wrap hidden">
            <button id="btnMobileBack" type="button" class="btn mobile-back-btn">
              <svg class="hi-icon" aria-hidden="true"><use href="#hi-x-mark"></use></svg>
              <span data-i18n="mobile_back_list">Quay lại danh sách</span>
            </button>
            <h2 id="readerSubject"></h2>
            <div class="reader-meta-row">
              <div id="readerMeta" class="reader-sender-block"></div>
              <span id="readerEntities"></span>
            </div>
            <div id="readerPhishing" class="phishing-box hidden">
              <svg class="hi-icon-lg" aria-hidden="true"><use href="#hi-shield-exclamation"></use></svg>
              <span id="readerPhishingText"></span>
            </div>
            <div id="readerActions" class="reader-actions">
              <button type="button" class="btn" data-act="archive">
                <svg class="hi-icon" aria-hidden="true"><use href="#hi-archive-box"></use></svg>
                <span data-i18n="act_archive">Lưu trữ</span>
              </button>
              <button type="button" class="btn" data-act="mark_read">
                <svg class="hi-icon" aria-hidden="true"><use href="#hi-envelope-open"></use></svg>
                <span data-i18n="act_read">Đã đọc</span>
              </button>
              <button type="button" class="btn" data-act="pin">
                <svg class="hi-icon" aria-hidden="true"><use href="#hi-bookmark"></use></svg>
                <span data-i18n="act_pin">Ghim</span>
              </button>
              <button type="button" id="btnOpenBodyLink" class="btn btn-primary hidden" data-i18n="open_body_link">Mở liên kết trong thư</button>
              <button type="button" id="btnUnsubscribe" class="btn hidden" data-i18n="act_unsub">Hủy đăng ký</button>
              <button type="button" id="btnDelete" class="btn btn-danger" data-act="delete">
                <svg class="hi-icon" aria-hidden="true"><use href="#hi-trash"></use></svg>
                <span data-i18n="act_delete">Xóa</span>
              </button>
            </div>
            <iframe id="readerHtmlFrame" class="reader-html-frame hidden" sandbox="allow-popups allow-popups-to-escape-sandbox" title="Nội dung email"></iframe>
            <div id="readerBody" class="reader-body"></div>
          </div>
        </section>
      </div>
    </main>

    <div id="toastBar" class="toast-bar" role="status" aria-live="polite">
      <svg class="hi-icon" aria-hidden="true"><use href="#hi-check-circle"></use></svg>
      <span id="toastText"></span>
    </div>
  </div>

  <script src="/app.js"></script>
</body>
</html>
"##;

pub const APP_JS: &str = r##"(() => {
  const I18N = {
    vi: {
      sync_now: "Làm mới",
      connect_account: "Thêm hộp thư",
      copy_code: "Sao chép mã",
      copy_all_codes: "Sao chép tất cả",
      open_verify_link: "Mở liên kết xác nhận",
      open_body_link: "Mở liên kết trong thư",
      mobile_back_list: "Quay lại danh sách",
      setup_heading: "Thêm hộp thư",
      setup_note: "Kết nối một hoặc nhiều hộp thư để tự động lấy mã xác nhận (OTP) cùng lúc.",
      mode_single: "Thêm 1 tài khoản",
      mode_bulk: "Thêm nhanh nhiều tài khoản",
      bulk_label: "Danh sách tài khoản (mỗi dòng 1 tài khoản)",
      bulk_hint: "Định dạng: email|mật_khẩu_ứng_dụng hoặc email:mật_khẩu. Hệ thống tự nhận diện máy chủ Gmail, Outlook, Hotmail, iCloud, Yahoo và đồng bộ song song tất cả tài khoản.",
      bulk_submit: "Kết nối và lấy mã tất cả",
      sync_this_acc: "Lấy mã TK này",
      all_accounts: "Gộp tất cả hộp thư",
      add_account_menu: "Thêm hộp thư mới",
      remove_all_accounts: "Gỡ tất cả",
      remove_account_tip: "Gỡ hộp thư này",
      google_login_btn: "Đăng nhập nhanh bằng Google",
      manual_divider: "Hoặc chọn dịch vụ để nhập mật khẩu ứng dụng thủ công:",
      back_to_inbox: "Quay lại hộp thư",
      preset_custom: "Email khác",
      email_label: "Địa chỉ Email",
      host_label: "Máy chủ",
      port_label: "Cổng",
      secret_label: "Mật khẩu ứng dụng",
      show_pw: "Hiện",
      hide_pw: "Ẩn",
      save_connect: "Kết nối hộp thư",
      cat_all: "Tất cả",
      cat_otp: "Mã OTP",
      cat_billing: "Hóa đơn",
      cat_shipping: "Đơn hàng",
      cat_security: "Bảo mật",
      cat_work: "Công việc",
      cat_promo: "Khuyến mãi",
      auto_sort: "Tự động sắp xếp",
      theme_dark: "Tối",
      theme_light: "Sáng",
      theme_toggle_tip: "Đổi giao diện Sáng / Tối",
      sfx_on_tip: "Âm thanh Creamy: Đang bật (Bấm để tắt)",
      sfx_off_tip: "Âm thanh Creamy: Đang tắt (Bấm để bật)",
      select_item_prompt: "Chọn một thư ở bên trái để đọc nội dung",
      act_read: "Đã đọc",
      act_archive: "Lưu trữ",
      act_pin: "Ghim",
      act_unsub: "Hủy đăng ký",
      act_delete: "Xóa",
      empty_no_items_title: "Chưa có thư nào trong mục này",
      empty_no_items_desc: "Bấm Làm mới hoặc chọn mục khác ở thanh phía trên.",
      search_placeholder: "Tìm kiếm thư, người gửi hoặc mã xác nhận...",
      guide_gmail: "Mẹo: Với Gmail, dùng Mật khẩu ứng dụng 16 ký tự trong phần Bảo mật của tài khoản Google.",
      guide_outlook: "Mẹo: Nhập địa chỉ Outlook/Hotmail và mật khẩu ứng dụng Microsoft của bạn.",
      guide_icloud: "Mẹo: Tạo mật khẩu ứng dụng tại trang quản lý Apple ID (appleid.apple.com).",
      guide_custom_imap: "Mẹo: Điền địa chỉ email, mật khẩu và thông tin máy chủ IMAP của bạn.",
      win_min: "Thu nhỏ",
      win_max: "Phóng to",
      win_close: "Đóng"
    },
    en: {
      sync_now: "Refresh",
      connect_account: "Add mailbox",
      copy_code: "Copy code",
      copy_all_codes: "Copy all",
      open_verify_link: "Open verification link",
      open_body_link: "Open link in email",
      mobile_back_list: "Back to list",
      setup_heading: "Add mailbox",
      setup_note: "Connect one or more mailboxes to fetch verification codes simultaneously.",
      mode_single: "Add single account",
      mode_bulk: "Bulk add accounts",
      bulk_label: "Account list (one account per line)",
      bulk_hint: "Format: email|app_password or email:password. Server settings for Gmail, Outlook, Hotmail, iCloud, and Yahoo are detected automatically and synced in parallel.",
      bulk_submit: "Connect and fetch all codes",
      sync_this_acc: "Fetch this account",
      all_accounts: "All mailboxes",
      add_account_menu: "Add mailbox",
      remove_all_accounts: "Remove all",
      remove_account_tip: "Remove this mailbox",
      google_login_btn: "Sign in with Google",
      manual_divider: "Or pick a service to connect manually:",
      back_to_inbox: "Back to inbox",
      preset_custom: "Other email",
      email_label: "Email address",
      host_label: "Server",
      port_label: "Port",
      secret_label: "App password",
      show_pw: "Show",
      hide_pw: "Hide",
      save_connect: "Connect mailbox",
      cat_all: "All",
      cat_otp: "OTP codes",
      cat_billing: "Billing",
      cat_shipping: "Orders",
      cat_security: "Security",
      cat_work: "Work",
      cat_promo: "Promotions",
      auto_sort: "Auto-sort",
      theme_dark: "Dark",
      theme_light: "Light",
      theme_toggle_tip: "Switch Light / Dark theme",
      sfx_on_tip: "Creamy SFX: On (Click to mute)",
      sfx_off_tip: "Creamy SFX: Off (Click to unmute)",
      select_item_prompt: "Select a message on the left to read",
      act_read: "Mark read",
      act_archive: "Archive",
      act_pin: "Pin",
      act_unsub: "Unsubscribe",
      act_delete: "Delete",
      empty_no_items_title: "No messages here yet",
      empty_no_items_desc: "Click Refresh or choose another filter tab above.",
      search_placeholder: "Search messages, senders, or OTP codes...",
      guide_gmail: "Tip: For Gmail, use a 16-character App Password from your Google Account Security settings.",
      guide_outlook: "Tip: Enter your Outlook email and Microsoft App Password.",
      guide_icloud: "Tip: Generate an app-specific password at appleid.apple.com.",
      guide_custom_imap: "Tip: Enter your email, password, and IMAP server details below.",
      win_min: "Minimize",
      win_max: "Maximize",
      win_close: "Close"
    }
  };

  const CAT_LABELS = {
    vi: {
      code_otp: "Mã OTP",
      billing: "Hóa đơn",
      shipping: "Đơn hàng",
      security_alert: "Bảo mật",
      work: "Công việc",
      social: "Mạng xã hội",
      promo_newsletter: "Khuyến mãi",
      other: "Thư khác"
    },
    en: {
      code_otp: "OTP",
      billing: "Billing",
      shipping: "Order",
      security_alert: "Security",
      work: "Work",
      social: "Social",
      promo_newsletter: "Promo",
      other: "General"
    }
  };

  let lang = "vi";
  let theme = "light";
  let sfxEnabled = true;
  try {
    const savedTheme = localStorage.getItem("vortex_theme");
    if (savedTheme === "dark" || savedTheme === "light") {
      theme = savedTheme;
    } else if (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches) {
      theme = "dark";
    }
    const savedSfx = localStorage.getItem("vortex_sfx");
    if (savedSfx === "0") {
      sfxEnabled = false;
    }
  } catch (_) {}

  const CUSTOM_SFX_B64 = "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";
  let audioCtx = null, customSfxBuffer = null, customSfxLoading = false, sfxFallbackAudio = null, lastSfxAt = 0;
  function ensureAudioCtx() {
    if (!audioCtx) {
      const Ctx = window.AudioContext || window.webkitAudioContext;
      if (!Ctx) return null;
      audioCtx = new Ctx();
    }
    if (audioCtx.state === "suspended") audioCtx.resume().catch(() => {});
    if (!customSfxBuffer && !customSfxLoading && audioCtx) {
      customSfxLoading = true;
      try {
        const bin = atob(CUSTOM_SFX_B64), bytes = new Uint8Array(bin.length);
        for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
        audioCtx.decodeAudioData(bytes.buffer.slice(0), (d) => { customSfxBuffer = d; }, () => {});
      } catch (_) {}
    }
    return audioCtx;
  }
  function playSampleSlice(ctx, offsetSec, durSec, rate, gainVal, delaySec = 0) {
    if (!customSfxBuffer) {
      if (!sfxFallbackAudio) sfxFallbackAudio = new Audio("data:audio/mp4;base64," + CUSTOM_SFX_B64);
      const clone = sfxFallbackAudio.cloneNode();
      clone.volume = Math.min(1, gainVal); clone.playbackRate = rate; clone.play().catch(() => {});
      return;
    }
    const t0 = ctx.currentTime + delaySec, src = ctx.createBufferSource();
    src.buffer = customSfxBuffer; src.playbackRate.setValueAtTime(rate, t0);
    const vca = ctx.createGain(), playDur = Math.min(durSec, Math.max(0.04, customSfxBuffer.duration - offsetSec));
    vca.gain.setValueAtTime(gainVal, t0);
    vca.gain.setValueAtTime(gainVal, t0 + Math.max(0.01, playDur - 0.025));
    vca.gain.exponentialRampToValueAtTime(0.0001, t0 + playDur);
    src.connect(vca); vca.connect(ctx.destination); src.start(t0, offsetSec, playDur);
  }
  function playCreamySfx(kind = "click") {
    if (!sfxEnabled) return;
    const nowMs = performance.now();
    if (nowMs - lastSfxAt < (kind === "pill" ? 45 : 24)) return;
    lastSfxAt = nowMs;
    const ctx = ensureAudioCtx();
    if (!ctx) return;
    const j = 0.985 + Math.random() * 0.03;
    if (kind === "pill") playSampleSlice(ctx, 0, 0.34, 1.04 * j, 0.9);
    else if (kind === "select") playSampleSlice(ctx, 0, 0.32, 0.96 * j, 0.82);
    else if (kind === "copy") { playSampleSlice(ctx, 0, 0.28, j, 0.88, 0); playSampleSlice(ctx, 0, 0.34, 1.18 * j, 0.92, 0.055); }
    else if (kind === "delete") playSampleSlice(ctx, 0, 0.34, 0.84 * j, 0.9);
    else playSampleSlice(ctx, 0, 0.34, j, 0.9);
  }
  let items = [];
  let cachedAllItems = [];
  let recentCodes = [];
  let hiddenItemKeys = new Set();
  try {
    const rawHidden = JSON.parse(localStorage.getItem("vortex_hidden_items") || "[]");
    if (Array.isArray(rawHidden)) {
      rawHidden.forEach((k) => hiddenItemKeys.add(String(k)));
    }
  } catch (_) {}

  function itemHideKeys(it) {
    if (!it) return [];
    const keys = [`id:${it.id}`];
    if (it.account || it.external_id) {
      keys.push(`ext:${it.account || ""}:${it.external_id || ""}`);
    }
    return keys;
  }

  function markItemHidden(it) {
    itemHideKeys(it).forEach((k) => hiddenItemKeys.add(k));
    try {
      const arr = Array.from(hiddenItemKeys).slice(-500);
      localStorage.setItem("vortex_hidden_items", JSON.stringify(arr));
    } catch (_) {}
  }

  function isItemVisible(it) {
    if (!it) return false;
    if (it.state === "trash" || it.state === "archived" || it.state === "spam") return false;
    return !itemHideKeys(it).some((k) => hiddenItemKeys.has(k));
  }
  let accounts = [];
  let selectedId = null;
  let activeCategory = "";
  let activeAccount = "";
  let activePreset = "gmail";
  let addMode = "single";
  let searchTimer = null;
  let toastTimer = null;
  let lastStreamKey = "";
  let lastRenderedItemId = null;
  let lastRenderedHtmlLen = -1;
  let autoEnrichedOnce = false;
  let syncInFlight = false;
  let autoSortEnabled = false;
  let prevCatIndex = 0;
  let catMorphTimer = null;
  let lastStreamMorphAt = 0;
  let lastReaderAnimAt = 0;
  const ANIM_COOLDOWN_MS = 380;

  const $ = (id) => document.getElementById(id);

  function t(key) {
    return (I18N[lang] && I18N[lang][key]) || key;
  }

  function catLabel(cat) {
    return (CAT_LABELS[lang] && CAT_LABELS[lang][cat]) || cat;
  }

  function showToast(msg) {
    const bar = $("toastBar");
    const txt = $("toastText");
    if (!bar || !txt) return;
    txt.textContent = msg;
    bar.classList.add("is-visible");
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      bar.classList.remove("is-visible");
    }, 2200);
  }

  function cleanSenderName(raw) {
    if (!raw) return "Gmail";
    const s = String(raw).trim();
    const ltIdx = s.indexOf("<");
    if (ltIdx > 0) {
      return s.slice(0, ltIdx).replace(/^["']|["']$/g, "").trim();
    }
    return s.replace(/^<|>$/g, "").trim();
  }

  function cleanSenderEmail(raw) {
    if (!raw) return "";
    const m = String(raw).match(/<([^>]+)>/);
    if (m) return m[1].trim();
    const s = String(raw).trim();
    return s.includes("@") ? s : "";
  }

  function stripHtmlSourceToText(raw) {
    let s = String(raw || "");
    if (/&lt;(?:!doctype|html|head|style|body|table|div)/i.test(s)) {
      const ta = document.createElement("textarea"); ta.innerHTML = s; s = ta.value;
    }
    if (/<(?:!doctype|html|head|style|body|table|div)/i.test(s)) {
      try {
        const doc = new DOMParser().parseFromString(s, "text/html");
        doc.querySelectorAll("script,style,head,meta,title,link,[style*='display:none'],[style*='visibility:hidden']").forEach((n) => n.remove());
        s = (doc.body && (doc.body.innerText || doc.body.textContent)) || s;
      } catch (_) {}
    }
    return s.replace(/\s+/g, " ").trim();
  }
  function cleanBodyText(subject, body) {
    let text = stripHtmlSourceToText(body);
    const subj = String(subject || "").trim();
    if (!text) return "";
    if (subj) {
      const low = subj.toLowerCase();
      while (text.toLowerCase().startsWith(low)) text = text.slice(subj.length).replace(/^[\s\u00a0—–\-:]+/, "").trim();
      const parts = subj.split(" - ");
      if (parts.length > 1) {
        const tail = parts[parts.length - 1].trim();
        if (tail.length > 4 && text.toLowerCase().startsWith(tail.toLowerCase())) text = text.slice(tail.length).replace(/^[\s\u00a0—–\-:]+/, "").trim();
      }
    }
    return text || subj;
  }

  function svgIcon(name, cls = "hi-icon") {
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", cls);
    svg.setAttribute("aria-hidden", "true");
    const use = document.createElementNS("http://www.w3.org/2000/svg", "use");
    use.setAttribute("href", "#" + name);
    svg.appendChild(use);
    return svg;
  }

  function setScreenMode(mode) {
    const isSetup = mode === "setup";
    $("onboardingView").classList.toggle("hidden", !isSetup);
    $("inboxView").classList.toggle("hidden", isSetup);
    $("topSearchWrap").classList.toggle("hidden", isSetup);
    $("btnCloseSetup").classList.toggle("hidden", accounts.length === 0);
  }

  function setAddMode(mode) {
    addMode = mode === "bulk" ? "bulk" : "single";
    $("btnModeSingle").setAttribute("aria-pressed", addMode === "single" ? "true" : "false");
    $("btnModeBulk").setAttribute("aria-pressed", addMode === "bulk" ? "true" : "false");
    $("singleAddWrap").classList.toggle("hidden", addMode !== "single");
    $("bulkAccountForm").classList.toggle("hidden", addMode !== "bulk");
    $("setupErrorBox").classList.add("hidden");
  }

  function selectPreset(preset) {
    activePreset = preset;
    $("accPreset").value = preset;
    document.querySelectorAll(".provider-grid .preset-btn").forEach((btn) => {
      btn.setAttribute("aria-pressed", btn.getAttribute("data-preset") === preset ? "true" : "false");
    });

    let showServer = false;
    if (preset === "gmail") {
      $("accHost").value = "imap.gmail.com";
      $("accPort").value = "993";
      $("accEmail").placeholder = "tenban@gmail.com";
    } else if (preset === "outlook") {
      $("accHost").value = "outlook.office365.com";
      $("accPort").value = "993";
      $("accEmail").placeholder = "tenban@outlook.com";
    } else if (preset === "icloud") {
      $("accHost").value = "imap.mail.me.com";
      $("accPort").value = "993";
      $("accEmail").placeholder = "tenban@icloud.com";
    } else {
      showServer = true;
    }

    $("customServerWrap").classList.toggle("hidden", !showServer);
    $("providerGuide").textContent = t("guide_" + preset);
  }

  function closeAccountMenu() {
    const menu = $("acctMenu");
    const btn = $("btnAcctMenu");
    if (menu) menu.classList.add("hidden");
    if (btn) btn.setAttribute("aria-expanded", "false");
  }

  function updateSetupButtonLabel() {
    const lbl = $("btnToggleSetupLabel");
    if (lbl) lbl.textContent = t("connect_account");
    const switcher = $("acctSwitcher");
    const activeLbl = $("acctActiveLabel");
    if (!switcher || !activeLbl) return;
    if (accounts.length === 0) {
      switcher.classList.add("hidden");
      closeAccountMenu();
      return;
    }
    switcher.classList.remove("hidden");
    if (activeAccount) {
      const found = accounts.find((a) => a.id === activeAccount);
      activeLbl.textContent = found ? found.email_or_handle : activeAccount;
    } else if (accounts.length === 1) {
      activeLbl.textContent = accounts[0].email_or_handle;
    } else {
      activeLbl.textContent = `${t("all_accounts")} (${accounts.length})`;
    }
  }

  function updateCategoryGlider() {
    const glider = $("catGlider");
    const activeBtn = document.querySelector('.category-pills .cat-btn[aria-pressed="true"]');
    if (!glider || !activeBtn) return;
    const w = activeBtn.offsetWidth;
    const h = activeBtn.offsetHeight;
    if (w <= 0 || h <= 0) {
      activeBtn.classList.add("is-glider-fallback");
      return;
    }
    activeBtn.classList.remove("is-glider-fallback");
    glider.style.width = `${w}px`;
    glider.style.height = `${h}px`;
    glider.style.transform = `translate3d(${activeBtn.offsetLeft}px, ${activeBtn.offsetTop}px, 0)`;
    glider.style.opacity = "1";
  }

  function triggerStreamMorph(dir = 1) {
    const ul = $("streamList");
    if (!ul) return;
    const now = performance.now();
    if (now - lastStreamMorphAt < ANIM_COOLDOWN_MS) {
      return;
    }
    lastStreamMorphAt = now;
    ul.style.setProperty("--cat-dir", String(dir));
    ul.classList.remove("is-cat-switching");
    void ul.offsetWidth;
    ul.classList.add("is-cat-switching");
    if (catMorphTimer) clearTimeout(catMorphTimer);
    catMorphTimer = setTimeout(() => {
      ul.classList.remove("is-cat-switching");
    }, 820);
  }

  function sortItemsList(list) {
    const copy = list.slice();
    if (!autoSortEnabled) {
      copy.sort((a, b) => {
        if (Boolean(a.pinned) !== Boolean(b.pinned)) return a.pinned ? -1 : 1;
        return (b.received_at || 0) - (a.received_at || 0);
      });
      return copy;
    }
    const catRank = {
      code_otp: 6,
      security_alert: 5,
      billing: 4,
      shipping: 3,
      work: 2,
      social: 1,
      other: 1,
      promo_newsletter: 0
    };
    copy.sort((a, b) => {
      if (Boolean(a.pinned) !== Boolean(b.pinned)) return a.pinned ? -1 : 1;
      const aHasOtp = a.category === "code_otp" || (a.extracted_codes && a.extracted_codes.length > 0);
      const bHasOtp = b.category === "code_otp" || (b.extracted_codes && b.extracted_codes.length > 0);
      if (aHasOtp !== bHasOtp) return aHasOtp ? -1 : 1;
      const aUnread = a.state === "unread";
      const bUnread = b.state === "unread";
      if (aUnread !== bUnread) return aUnread ? -1 : 1;
      const aRank = catRank[a.category] !== undefined ? catRank[a.category] : 1;
      const bRank = catRank[b.category] !== undefined ? catRank[b.category] : 1;
      if (aRank !== bRank) return bRank - aRank;
      return (b.received_at || 0) - (a.received_at || 0);
    });
    return copy;
  }

  function applySfxButton() {
    const iconUse = $("sfxIconUse");
    if (iconUse) {
      iconUse.setAttribute("href", sfxEnabled ? "#hi-speaker-wave" : "#hi-speaker-x-mark");
    }
    const btn = $("btnSfx");
    if (btn) {
      const tip = sfxEnabled ? t("sfx_on_tip") : t("sfx_off_tip");
      btn.setAttribute("aria-label", tip);
      btn.title = tip;
      btn.style.opacity = sfxEnabled ? "1" : "0.55";
    }
  }

  function applyTheme(rerenderIframe = false) {
    document.documentElement.setAttribute("data-theme", theme);
    const iconUse = $("themeIconUse");
    if (iconUse) {
      iconUse.setAttribute("href", theme === "dark" ? "#hi-sun" : "#hi-moon");
    }
    const lbl = $("themeLabel");
    if (lbl) {
      lbl.textContent = theme === "dark" ? t("theme_light") : t("theme_dark");
    }
    const btn = $("btnTheme");
    if (btn) {
      btn.setAttribute("aria-label", t("theme_toggle_tip"));
      btn.title = t("theme_toggle_tip");
    }
    applySfxButton();
    if (rerenderIframe && selectedId) {
      lastRenderedItemId = null;
      selectItem(selectedId, false);
    }
    requestAnimationFrame(updateCategoryGlider);
  }

  function applyTranslations() {
    document.documentElement.lang = lang;
    $("btnLang").setAttribute("aria-label", lang === "vi" ? "Switch to English" : "Chuyển sang tiếng Việt");
    $("btnSync").setAttribute("aria-label", t("sync_now"));
    $("btnSync").title = t("sync_now");
    $("searchInput").setAttribute("aria-label", t("search_placeholder"));
    $("langLabel").textContent = lang === "vi" ? "EN" : "VI";
    if ($("btnWinMin")) {
      $("btnWinMin").setAttribute("aria-label", t("win_min"));
      $("btnWinMin").title = t("win_min");
    }
    if ($("btnWinMax")) {
      $("btnWinMax").setAttribute("aria-label", t("win_max"));
      $("btnWinMax").title = t("win_max");
    }
    if ($("btnWinClose")) {
      $("btnWinClose").setAttribute("aria-label", t("win_close"));
      $("btnWinClose").title = t("win_close");
    }
    applyTheme(false);
    document.querySelectorAll("[data-i18n]").forEach((el) => {
      const k = el.getAttribute("data-i18n");
      if (I18N[lang][k]) el.textContent = I18N[lang][k];
    });
    updateSetupButtonLabel();
    $("searchInput").placeholder = t("search_placeholder");
    $("providerGuide").textContent = t("guide_" + activePreset);
    lastStreamKey = "";
    renderAccountSwitcher(cachedAllItems);
    renderStream(false);
    if (selectedId) selectItem(selectedId, false);
    requestAnimationFrame(updateCategoryGlider);
  }

  async function api(path, opts = {}) {
    const res = await fetch(path, {
      headers: { "Content-Type": "application/json" },
      ...opts
    });
    if (!res.ok) {
      const txt = await res.text();
      throw new Error(txt || res.statusText);
    }
    return res.json();
  }

  function updateCategoryCounts(allItems) {
    const scoped = activeAccount
      ? allItems.filter((it) => it.account === activeAccount)
      : allItems;
    const counts = {
      all: scoped.length,
      code_otp: 0,
      billing: 0,
      shipping: 0,
      security_alert: 0,
      work: 0,
      promo_newsletter: 0
    };
    scoped.forEach((it) => {
      if (counts[it.category] !== undefined) counts[it.category] += 1;
    });
    let changed = false;
    Object.keys(counts).forEach((k) => {
      const el = $("cnt_" + k);
      if (el) {
        const next = counts[k] > 0 ? String(counts[k]) : "";
        if (el.textContent !== next) {
          el.textContent = next;
          changed = true;
        }
      }
    });
    if (changed) requestAnimationFrame(updateCategoryGlider);
  }

  async function loadAll(forceStreamAnim = false) {
    try {
      const qs = [];
      if (activeCategory) qs.push(`category=${encodeURIComponent(activeCategory)}`);
      if (activeAccount) qs.push(`account=${encodeURIComponent(activeAccount)}`);
      const filterUrl = "/api/items" + (qs.length ? `?${qs.join("&")}` : "");

      const [status, accs, rawAllItems, rawFilteredList, rawCodes] = await Promise.all([
        api("/api/status"),
        api("/api/accounts"),
        api("/api/items"),
        api(filterUrl),
        api("/api/codes")
      ]);
      const allItems = rawAllItems.filter(isItemVisible);
      const filteredList = rawFilteredList.filter(isItemVisible);
      const codes = rawCodes.filter((c) => !hiddenItemKeys.has(`id:${c.item_id}`));
      accounts = accs;
      if (activeAccount && !accounts.some((a) => a.id === activeAccount)) {
        activeAccount = "";
      }
      cachedAllItems = allItems;
      items = sortItemsList(filteredList);
      recentCodes = codes;
      updateSetupButtonLabel();
      updateCategoryCounts(allItems);
      if (accounts.length === 0) {
        setScreenMode("setup");
      } else if (status.missing_secret_accounts && status.missing_secret_accounts.length > 0 && rawAllItems.length === 0) {
        const missingEmail = status.missing_secret_accounts[0];
        setScreenMode("setup");
        setAddMode("single");
        $("accEmail").value = missingEmail;
        $("setupErrorText").textContent = lang === "vi"
          ? `Vui lòng nhập Mật khẩu ứng dụng cho ${missingEmail} để tải thư về.`
          : `Please enter the App Password for ${missingEmail} to sync your messages.`;
        $("setupErrorBox").classList.remove("hidden");
      } else if (!$("onboardingView").classList.contains("hidden") && selectedId) {
        setScreenMode("inbox");
      }
      renderAccountSwitcher(allItems);
      if (forceStreamAnim) triggerStreamMorph(1);
      renderStream(forceStreamAnim);
      if (items.length === 0) {
        selectedId = null;
        lastRenderedItemId = null;
        $("readerContentWrap").classList.add("hidden");
        $("readerEmptyState").classList.remove("hidden");
      } else if (forceStreamAnim || !selectedId || !items.some((x) => x.id === selectedId)) {
        if (window.innerWidth > 720) {
          selectItem(items[0].id, false);
        }
      } else if (selectedId) {
        selectItem(selectedId, false);
      }
  
      if (!autoEnrichedOnce && allItems.length > 0 && allItems.some((i) => !i.body_html || (i.body_text || "").length < 280)) {
        autoEnrichedOnce = true;
        triggerSync(null, true);
      }
    } catch (err) {
      $("statusNotice").textContent = String(err.message || err);
    }
  }

  let syncSuccessTimer = null;

  async function triggerSync(accountId = null, silent = false) {
    if (syncInFlight) return;
    syncInFlight = true;
    const btn = $("btnSync");
    const useEl = btn ? btn.querySelector("use") : null;
    if (!silent && btn) {
      if (syncSuccessTimer) {
        clearTimeout(syncSuccessTimer);
        syncSuccessTimer = null;
      }
      btn.classList.remove("is-synced");
      if (useEl) useEl.setAttribute("href", "#hi-arrow-path");
      btn.classList.add("is-spinning");
    }
    try {
      const payload = accountId ? { account_id: accountId } : {};
      const res = await api("/api/sync", { method: "POST", body: JSON.stringify(payload) });
      await loadAll();
      if (!silent && btn && !res.error) {
        btn.classList.remove("is-spinning");
        if (useEl) useEl.setAttribute("href", "#hi-check-circle");
        btn.classList.add("is-synced");
        syncSuccessTimer = setTimeout(() => {
          btn.classList.remove("is-synced");
          if (useEl) useEl.setAttribute("href", "#hi-arrow-path");
          syncSuccessTimer = null;
        }, 1200);
      }
    } catch (err) {
      // silent
    } finally {
      syncInFlight = false;
      if (btn) btn.classList.remove("is-spinning");
    }
  }

  function renderAccountSwitcher(allItems = []) {
    const menu = $("acctMenu");
    if (!menu) return;
    menu.innerHTML = "";
    if (accounts.length === 0) return;

    const list = document.createElement("div");
    list.className = "acct-menu-list";

    const nowSec = Math.floor(Date.now() / 1000);

    if (accounts.length > 1) {
      const allWrap = document.createElement("div");
      allWrap.className = "acct-item-wrap";
      allWrap.setAttribute("data-active", activeAccount === "" ? "true" : "false");

      const allBtn = document.createElement("button");
      allBtn.type = "button";
      allBtn.className = "acct-item-btn";

      const allMain = document.createElement("div");
      allMain.className = "acct-item-main";
      const allText = document.createElement("div");
      allText.className = "acct-item-text";
      const allTitle = document.createElement("span");
      allTitle.className = "acct-item-email";
      allTitle.textContent = `${t("all_accounts")} (${accounts.length})`;
      const allSub = document.createElement("span");
      allSub.className = "acct-item-sub";
      allSub.textContent = lang === "vi" ? `${allItems.length} thư tổng cộng` : `${allItems.length} messages total`;
      allText.append(allTitle, allSub);
      allMain.append(svgIcon(activeAccount === "" ? "hi-check-circle" : "hi-user-group"), allText);
      allBtn.appendChild(allMain);

      allBtn.onclick = () => {
        closeAccountMenu();
        if (activeAccount === "" && $("onboardingView").classList.contains("hidden")) return;
        activeAccount = "";
        setScreenMode("inbox");
        loadAll(true);
      };
      allWrap.appendChild(allBtn);
      list.appendChild(allWrap);
    }

    accounts.forEach((a) => {
      const isSelected = activeAccount === a.id || (accounts.length === 1 && activeAccount === "");
      const wrap = document.createElement("div");
      wrap.className = "acct-item-wrap";
      wrap.setAttribute("data-active", isSelected ? "true" : "false");

      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "acct-item-btn";

      const main = document.createElement("div");
      main.className = "acct-item-main";

      const textWrap = document.createElement("div");
      textWrap.className = "acct-item-text";

      const emailSpan = document.createElement("span");
      emailSpan.className = "acct-item-email";
      emailSpan.textContent = a.email_or_handle;

      const mailCnt = allItems.filter((it) => it.account === a.id).length;
      const activeOtp = recentCodes.find((c) => c.expires_at > nowSec && c.account === a.id);
      const subSpan = document.createElement("span");
      subSpan.className = "acct-item-sub";
      if (activeOtp) {
        subSpan.textContent = lang === "vi"
          ? `Mã OTP: ${activeOtp.code} · ${mailCnt} thư`
          : `OTP: ${activeOtp.code} · ${mailCnt} msgs`;
      } else {
        subSpan.textContent = lang === "vi" ? `${mailCnt} thư` : `${mailCnt} messages`;
      }

      textWrap.append(emailSpan, subSpan);
      main.append(svgIcon(isSelected ? "hi-check-circle" : "hi-user-circle"), textWrap);
      btn.appendChild(main);

      btn.onclick = () => {
        const nextAcc = accounts.length === 1 ? "" : a.id;
        closeAccountMenu();
        if (activeAccount === nextAcc && $("onboardingView").classList.contains("hidden")) return;
        activeAccount = nextAcc;
        setScreenMode("inbox");
        loadAll(true);
      };

      const delBtn = document.createElement("button");
      delBtn.type = "button";
      delBtn.className = "acct-item-del";
      delBtn.title = t("remove_account_tip");
      delBtn.setAttribute("aria-label", t("remove_account_tip"));
      delBtn.appendChild(svgIcon("hi-trash"));
      delBtn.onclick = async (e) => {
        e.stopPropagation();
        if (activeAccount === a.id) activeAccount = "";
        await api("/api/accounts/remove", { method: "POST", body: JSON.stringify({ id: a.id }) });
        await loadAll();
      };

      wrap.append(btn, delBtn);
      list.appendChild(wrap);
    });

    menu.appendChild(list);

    const foot = document.createElement("div");
    foot.className = "acct-menu-foot";

    const addBtn = document.createElement("button");
    addBtn.type = "button";
    addBtn.className = "btn btn-sm";
    addBtn.append(svgIcon("hi-user-plus"), document.createTextNode(t("add_account_menu")));
    addBtn.onclick = () => {
      closeAccountMenu();
      setScreenMode("setup");
    };
    foot.appendChild(addBtn);

    if (accounts.length > 1) {
      const clearBtn = document.createElement("button");
      clearBtn.type = "button";
      clearBtn.className = "btn btn-sm btn-danger";
      clearBtn.append(svgIcon("hi-trash"), document.createTextNode(t("remove_all_accounts")));
      clearBtn.onclick = async () => {
        closeAccountMenu();
        await api("/api/accounts/remove", { method: "POST", body: JSON.stringify({ all: true }) });
        activeAccount = "";
        await loadAll();
      };
      foot.appendChild(clearBtn);
    }

    menu.appendChild(foot);
  }

  function createSenderAvatar(rawSender) {
    const avatar = document.createElement("span");
    avatar.className = "sender-avatar";
    const name = cleanSenderName(rawSender), email = cleanSenderEmail(rawSender);
    const domain = (email.split("@")[1] || "").toLowerCase();
    const initial = ((name || email || "V").trim().charAt(0) || "V").toUpperCase();
    let primaryUrl = "", secondaryUrl = "";
    if (domain === "github.com" || domain.endsWith(".github.com")) {
      const cleanHandle = name.replace(/\[bot\]$/i, "").trim();
      if (cleanHandle.toLowerCase() === "github" || email.startsWith("noreply@")) primaryUrl = "https://github.com/github.png?size=64";
      else if (/^[a-zA-Z0-9-]+$/.test(cleanHandle)) { primaryUrl = `https://github.com/${encodeURIComponent(cleanHandle)}.png?size=64`; secondaryUrl = "https://github.com/github.png?size=64"; }
      else primaryUrl = "https://github.com/github.png?size=64";
    } else if (domain && !/^(gmail\.com|googlemail\.com|outlook\.com|hotmail\.com|icloud\.com|yahoo\.com|live\.com)$/.test(domain)) {
      primaryUrl = `https://www.google.com/s2/favicons?domain=${encodeURIComponent(domain)}&sz=64`;
      secondaryUrl = `https://icons.duckduckgo.com/ip3/${encodeURIComponent(domain)}.ico`;
    } else if (email) primaryUrl = `https://unavatar.io/${encodeURIComponent(email)}?fallback=false`;
    const showInitial = () => {
      avatar.innerHTML = "";
      const initSpan = document.createElement("span");
      initSpan.className = "sender-avatar-initial";
      initSpan.textContent = initial;
      avatar.appendChild(initSpan);
    };
    if (!primaryUrl) { showInitial(); return avatar; }
    const img = document.createElement("img");
    img.className = "sender-avatar-img"; img.src = primaryUrl; img.alt = name; img.loading = "lazy";
    img.onerror = () => { if (secondaryUrl && img.src !== secondaryUrl) img.src = secondaryUrl; else showInitial(); };
    avatar.appendChild(img);
    return avatar;
  }
  function renderStream(force = false) {
    const ul = $("streamList");
    const nextKey = items.map((i) => `${i.id}:${i.state}:${i.pinned ? 1 : 0}:${(i.subject || "").length}:${(i.body_text || "").length}:${(i.body_html || "").length}`).join("|");
    if (!force && nextKey === lastStreamKey && ul.children.length > 0) { updateStreamSelectionAttr(); return; }
    lastStreamKey = nextKey;
    ul.innerHTML = "";
    if (!items.length) {
      const li = document.createElement("li"); li.className = "stream-empty";
      const icon = svgIcon("hi-inbox", "hi-icon-lg");
      const title = document.createElement("div"); title.className = "stream-empty-title"; title.textContent = t("empty_no_items_title");
      const desc = document.createElement("div"); desc.className = "stream-empty-desc"; desc.textContent = t("empty_no_items_desc");
      li.append(icon, title, desc); ul.appendChild(li); return;
    }
    items.forEach((item, idx) => {
      const li = document.createElement("li");
      li.className = "stream-row" + (item.state === "unread" ? " stream-row-unread" : "");
      li.style.setProperty("--row-idx", String(Math.min(idx, 10)));
      li.setAttribute("data-id", String(item.id));
      li.setAttribute("aria-selected", item.id === selectedId ? "true" : "false");
      li.tabIndex = 0;
      const header = document.createElement("div"); header.className = "row-header";
      const avatar = createSenderAvatar(item.sender);
      const info = document.createElement("div"); info.className = "sender-info";
      const topLine = document.createElement("div"); topLine.className = "sender-top-line";
      const senderNameSpan = document.createElement("span"); senderNameSpan.className = "row-sender-name";
      senderNameSpan.textContent = (item.pinned ? "[Ghim] " : "") + cleanSenderName(item.sender);
      const timeEl = document.createElement("span"); timeEl.className = "row-time";
      timeEl.textContent = new Date(item.received_at * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
      topLine.append(senderNameSpan, timeEl);
      const emailLine = document.createElement("div"); emailLine.className = "row-sender-email";
      const sEmail = cleanSenderEmail(item.sender);
      emailLine.textContent = (!activeAccount && item.account && sEmail && sEmail !== item.account) ? `${sEmail} · ${item.account}` : (sEmail || item.account || "");
      info.append(topLine, emailLine); header.append(avatar, info);
      const subj = document.createElement("div"); subj.className = "row-subject"; subj.textContent = item.subject || "(Không tiêu đề)";
      const snip = document.createElement("div"); snip.className = "row-snippet";
      snip.textContent = cleanBodyText(item.subject, item.summary || item.body_text || "").slice(0, 140);
      li.append(header, subj, snip);
      li.onclick = () => { playCreamySfx("select"); selectItem(item.id, true); };
      li.onkeydown = (e) => { if (e.key === "Enter") { playCreamySfx("select"); selectItem(item.id, true); } };
      ul.appendChild(li);
    });
  }
  function updateStreamSelectionAttr() {
    document.querySelectorAll("#streamList .stream-row").forEach((li) => {
      li.setAttribute("aria-selected", Number(li.getAttribute("data-id")) === selectedId ? "true" : "false");
    });
  }

  function parseEmailColor(raw) {
    if (!raw) return null;
    const s = String(raw).trim().toLowerCase().replace(/\s*!important\s*$/i, "");
    if (!s || /^(transparent|none|inherit|initial|unset|currentcolor)$/.test(s)) return null;
    const named = { white:[255,255,255],black:[0,0,0],gray:[128,128,128],grey:[128,128,128],silver:[192,192,192],whitesmoke:[245,245,245],lightgray:[211,211,211],lightgrey:[211,211,211],darkgray:[169,169,169],darkgrey:[169,169,169],dimgray:[105,105,105],dimgrey:[105,105,105],gainsboro:[220,220,220],blue:[0,0,255],navy:[0,0,128],red:[255,0,0],green:[0,128,0] };
    if (named[s]) return { r: named[s][0], g: named[s][1], b: named[s][2], a: 1 };
    const hm = s.match(/^#([0-9a-f]{3,8})$/i);
    if (hm) {
      const h = hm[1];
      if (h.length <= 4) return { r: parseInt(h[0]+h[0],16), g: parseInt(h[1]+h[1],16), b: parseInt(h[2]+h[2],16), a: h.length === 4 ? parseInt(h[3]+h[3],16)/255 : 1 };
      if (h.length === 6 || h.length === 8) return { r: parseInt(h.slice(0,2),16), g: parseInt(h.slice(2,4),16), b: parseInt(h.slice(4,6),16), a: h.length === 8 ? parseInt(h.slice(6,8),16)/255 : 1 };
    }
    const rm = s.match(/rgba?\(\s*([0-9.]+)\s*[, ]\s*([0-9.]+)\s*[, ]\s*([0-9.]+)(?:\s*[,/]\s*([0-9.]+%?))?\s*\)/i);
    if (rm) {
      let a = rm[4] !== undefined ? (rm[4].endsWith("%") ? parseFloat(rm[4])/100 : parseFloat(rm[4])) : 1;
      return { r: Math.min(255,Math.max(0,Math.round(parseFloat(rm[1])))), g: Math.min(255,Math.max(0,Math.round(parseFloat(rm[2])))), b: Math.min(255,Math.max(0,Math.round(parseFloat(rm[3])))), a: Math.min(1,Math.max(0,isNaN(a)?1:a)) };
    }
    return null;
  }
  function blendEmailColor(fg, bg) { const a = fg.a !== undefined ? fg.a : 1; return { r: Math.round(fg.r * a + bg.r * (1 - a)), g: Math.round(fg.g * a + bg.g * (1 - a)), b: Math.round(fg.b * a + bg.b * (1 - a)), a: 1 }; }
  function emailRelLum(c) { const ch = (v) => { const x = v / 255; return x <= 0.03928 ? x / 12.92 : Math.pow((x + 0.055) / 1.055, 2.4); }; return 0.2126 * ch(c.r) + 0.7152 * ch(c.g) + 0.0722 * ch(c.b); }
  function emailContrast(l1, l2) { return (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05); }
  function normalizeEmailDom(root, isDark) {
    if (!root) return;
    const baseBg = isDark ? { r: 27, g: 27, b: 25, a: 1 } : { r: 244, g: 244, b: 240, a: 1 };
    const altBgHex = isDark ? "#252522" : "#e5e5dd";
    const altBgRgb = isDark ? { r: 37, g: 37, b: 34, a: 1 } : { r: 229, g: 229, b: 221, a: 1 };
    const fgHex = isDark ? "#ecece6" : "#181816";
    const fgMutedHex = isDark ? "#c8c8c0" : "#363632";
    const borderHex = isDark ? "#66665e" : "#85857c";
    const isCellActive = (c) => ((c.textContent || "").trim().length > 0) || !!c.querySelector("img,svg,[role='img']");
    const tableCandidates = new Set();
    for (const tbl of Array.from(root.querySelectorAll("table"))) {
      const rows = Array.from(tbl.rows || []);
      if (!rows.length) continue;
      let multiColRows = 0, hasTh = false, wrappedRows = 0;
      for (const r of rows) {
        const cells = Array.from(r.cells || []);
        if (cells.some((c) => c.tagName === "TH")) hasTh = true;
        if (cells.filter(isCellActive).length >= 2) multiColRows++;
        else if (cells.length === 1) {
          const it = cells[0].querySelector(":scope > table");
          if (it && it.rows && it.rows.length === 1 && Array.from(it.rows[0].cells || []).filter(isCellActive).length >= 2) wrappedRows++;
        }
      }
      const bAttr = tbl.getAttribute("border");
      if (hasTh || multiColRows >= 2 || wrappedRows >= 2 || (bAttr && bAttr !== "0")) tableCandidates.add(tbl);
    }
    for (const tbl of Array.from(tableCandidates)) {
      if (Array.from(tbl.querySelectorAll("table")).some((sub) => tableCandidates.has(sub))) tableCandidates.delete(tbl);
    }
    const markTableRows = (tbl, rows, forceIdx0Head) => {
      tbl.classList.add("vortex-data-table");
      let p = tbl.parentElement;
      while (p && p !== root) {
        if ((p.getAttribute("style") && /border/i.test(p.getAttribute("style"))) || p.hasAttribute("border")) {
          p.removeAttribute("border");
          p.style.setProperty("border", "0", "important");
          p.style.setProperty("padding", "0", "important");
        }
        p = p.parentElement;
      }
      rows.forEach((r, rIdx) => {
        r.classList.add("vortex-data-row");
        let cells = Array.from(r.cells || []);
        if (cells.length === 1) {
          const it = cells[0].querySelector(":scope > table");
          if (it && it.rows && it.rows.length === 1 && Array.from(it.rows[0].cells || []).filter(isCellActive).length >= 2) {
            it.style.setProperty("width", "100%", "important");
            it.style.setProperty("border-collapse", "collapse", "important");
            cells = Array.from(it.rows[0].cells || []);
          }
        }
        const isHead = (forceIdx0Head || (rIdx === 0 && rows.length >= 2)) && (cells.some((c) => c.tagName === "TH") || cells.every((c) => (c.textContent || "").trim().length <= 32 && !c.querySelector("a,img")));
        cells.forEach((c) => { c.classList.add("vortex-data-cell"); if (isHead || c.tagName === "TH") c.classList.add("vortex-data-head-cell"); });
      });
    };
    root.querySelectorAll("td,div,section,article,center,body").forEach((box) => {
      const sibs = Array.from(box.children || []).filter((t) => t.tagName === "TABLE" && t.rows && t.rows.length === 1 && Array.from(t.rows[0].cells || []).filter(isCellActive).length >= 2);
      if (sibs.length >= 2) sibs.forEach((t, idx) => { t.classList.add("vortex-stacked-table"); markTableRows(t, [t.rows[0]], idx === 0); });
    });
    tableCandidates.forEach((tbl) => markTableRows(tbl, Array.from(tbl.rows || []), false));
    const walk = (el, parentBg, insideLink) => {
      if (!el || el.nodeType !== 1) return;
      const tag = el.tagName;
      let currentBg = parentBg;
      const rawStyle = el.getAttribute("style") || "";
      const parsedBg = parseEmailColor((el.style && el.style.backgroundColor) || el.getAttribute("bgcolor") || "");
      if (parsedBg && parsedBg.a > 0.05) {
        const blendedBg = blendEmailColor(parsedBg, parentBg);
        const bgLum = emailRelLum(blendedBg);
        const bgChroma = Math.max(blendedBg.r, blendedBg.g, blendedBg.b) - Math.min(blendedBg.r, blendedBg.g, blendedBg.b);
        if (isDark && bgLum > 0.32 && bgChroma <= 48) {
          el.removeAttribute("bgcolor");
          el.style.setProperty("background-color", bgLum > 0.86 ? "#1b1b19" : altBgHex, "important");
          currentBg = bgLum > 0.86 ? baseBg : altBgRgb;
        } else if (!isDark && bgLum < 0.16 && bgChroma <= 40 && /^(TABLE|TBODY|THEAD|TR|TD|TH|DIV|SECTION|ARTICLE|CENTER|BODY)$/.test(tag)) {
          el.removeAttribute("bgcolor");
          el.style.setProperty("background-color", "#f4f4f0", "important");
          currentBg = baseBg;
        } else currentBg = blendedBg;
      }
      if (rawStyle && /border/i.test(rawStyle)) {
        const bgLum = emailRelLum(currentBg);
        ["", "Top", "Right", "Bottom", "Left"].forEach((side) => {
          const val = el.style["border" + side + "Color"];
          if (val) {
            const bc = parseEmailColor(val);
            if (!bc || emailContrast(emailRelLum(blendEmailColor(bc, currentBg)), bgLum) < 2.5) el.style.setProperty("border-" + (side ? side.toLowerCase() + "-" : "") + "color", borderHex, "important");
          }
        });
        if (tag === "TR") { const pTbl = el.closest("table"); if (pTbl) pTbl.style.setProperty("border-collapse", "collapse", "important"); }
      }
      if (el.hasAttribute("bordercolor")) { el.removeAttribute("bordercolor"); el.style.setProperty("border-color", borderHex, "important"); }
      const bgLum = emailRelLum(currentBg);
      const bgChroma = Math.max(currentBg.r, currentBg.g, currentBg.b) - Math.min(currentBg.r, currentBg.g, currentBg.b);
      const isLink = tag === "A" || insideLink;
      const colorAttr = el.getAttribute("color");
      const parsedFg = parseEmailColor((el.style && el.style.color) || colorAttr || "");
      if (colorAttr) el.removeAttribute("color");
      if (isLink) {
        if (bgChroma > 48) {
          if (parsedFg) { if (emailContrast(emailRelLum(blendEmailColor(parsedFg, currentBg)), bgLum) < 3.8) el.style.setProperty("color", bgLum < 0.48 ? "#fafaf5" : "#181816", "important"); }
          else if (tag === "A") el.style.setProperty("color", bgLum < 0.48 ? "#fafaf5" : "#181816", "important");
        } else {
          const targetLinkHex = bgLum < 0.5 ? "#6cb6ff" : "#0b57d0";
          if (!parsedFg) { if (tag === "A") el.style.setProperty("color", targetLinkHex, "important"); }
          else {
            const fgBlended = blendEmailColor(parsedFg, currentBg);
            const fgChroma = Math.max(fgBlended.r, fgBlended.g, fgBlended.b) - Math.min(fgBlended.r, fgBlended.g, fgBlended.b);
            if (emailContrast(emailRelLum(fgBlended), bgLum) < 4.8 || fgChroma < 25) el.style.setProperty("color", targetLinkHex, "important");
          }
        }
      } else if (parsedFg) {
        const fgBlended = blendEmailColor(parsedFg, currentBg);
        if (emailContrast(emailRelLum(fgBlended), bgLum) < 4.8) {
          const isMuted = parsedFg.a < 0.72 || (emailRelLum(parsedFg) > 0.16 && emailRelLum(parsedFg) < 0.58);
          el.style.setProperty("color", isMuted ? fgMutedHex : fgHex, "important");
        }
      } else if ((isDark && bgLum >= 0.5) || (!isDark && bgLum < 0.5)) {
        el.style.setProperty("color", bgLum < 0.5 ? "#ecece6" : "#181816", "important");
      }
      for (let i = 0; i < el.children.length; i++) walk(el.children[i], currentBg, isLink);
    };
    walk(root, baseBg, false);
  }

  function buildEmailIframeDoc(rawHtml, plainText, animate = true) {
    const isDark = theme === "dark";
    let bodyContent = (rawHtml || "").replace(/\[remote image blocked\]/gi, "").trim();
    if (/&lt;(?:!doctype|html|head|style|body|table|div|p|span|a\b)/i.test(bodyContent)) {
      const ta = document.createElement("textarea"); ta.innerHTML = bodyContent; bodyContent = ta.value;
    }
    if (!bodyContent && /<(?:!doctype|html|head|style|body|table|div|p|span|a\b)/i.test(String(plainText || ""))) {
      bodyContent = String(plainText || "");
    }
    if (/<[a-z!][\s\S]*>/i.test(bodyContent)) {
      try {
        const doc = new DOMParser().parseFromString(bodyContent, "text/html");
        doc.querySelectorAll("script,style,head,meta,title,link,[style*='display:none'],[style*='visibility:hidden']").forEach((n) => n.remove());
        if (doc.body) {
          normalizeEmailDom(doc.body, isDark);
          if (doc.body.innerHTML.trim()) bodyContent = doc.body.innerHTML.trim();
        }
      } catch (_) {}
    }
    if (!bodyContent) {
      const escaped = String(plainText || "")
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/(https?:\/\/[^\s"'<>)]+)/g, '<a href="$1" target="_blank" rel="noopener noreferrer">$1</a>')
        .replace(/\n/g, "<br>");
      bodyContent = `<div style="line-height:1.65;font-size:15px;">${escaped}</div>`;
    }
    const bgHex = isDark ? "#1b1b19" : "#f4f4f0";
    const surfaceAltHex = isDark ? "#252522" : "#e5e5dd";
    const fgHex = isDark ? "#ecece6" : "#181816";
    const linkHex = isDark ? "#6cb6ff" : "#0b57d0";
    const borderHex = isDark ? "#66665e" : "#85857c";
    const borderStrongHex = isDark ? "#85857c" : "#5c5c55";
    const sbThumb = isDark ? "#4e4e48" : "#9e9e95";
    const sbHover = isDark ? "#7c7c74" : "#62625b";
    const sbActive = isDark ? "#ecece6" : "#1a1a18";
    const bodyAnimRule = animate
      ? "body { animation: sheetImmersion 680ms cubic-bezier(0.16, 1, 0.3, 1) both; }"
      : "";
    return `<!doctype html><html><head><meta charset="utf-8"><base target="_blank"><style>
      @font-face { font-family: "Vortex SF Pro"; src: local("SF Pro Display Bold"), local("SFProDisplay-Bold"); font-weight: 700; }
      @keyframes sheetImmersion { 0% { opacity: 0; transform: translate3d(0, 12px, 0); } 100% { opacity: 1; transform: translate3d(0, 0, 0); } }
      *, *::before, *::after { box-sizing: border-box; }
      *::-webkit-scrollbar { width: 8px; height: 0 !important; background: transparent; }
      *::-webkit-scrollbar:horizontal, *::-webkit-scrollbar-button, *::-webkit-scrollbar-corner { display: none !important; width: 0 !important; height: 0 !important; }
      *::-webkit-scrollbar-track { background: transparent; margin-block: 6px; }
      *::-webkit-scrollbar-thumb { background-color: ${sbThumb}; border-radius: 999px; border: 2px solid transparent; background-clip: padding-box; min-height: 36px; }
      *::-webkit-scrollbar-thumb:hover { background-color: ${sbHover}; }
      *::-webkit-scrollbar-thumb:active { background-color: ${sbActive}; }
      html, body { margin: 0; padding: 12px 6px 28px 4px; max-width: 100%; overflow-x: hidden !important; font-family: "Vortex SF Pro", -apple-system, system-ui, "Segoe UI", Roboto, sans-serif; font-weight: 700; font-size: 15px; line-height: 1.6; color: ${fgHex}; background-color: ${bgHex}; overflow-wrap: break-word; }
      ${bodyAnimRule}
      .blocked-image { display: none !important; }
      img { max-width: 100% !important; height: auto !important; }
      a { color: ${linkHex}; text-decoration: underline; text-underline-offset: 2px; overflow-wrap: break-word; }
      table { border-collapse: collapse; border-spacing: 0; }
      table, tbody, tr, td, th, div, section, article, center { max-width: 100% !important; }
      table.vortex-data-table { width: 100% !important; border-collapse: collapse !important; border-spacing: 0 !important; border: 1.5px solid ${borderStrongHex} !important; margin: 8px 0 !important; }
      table.vortex-stacked-table + table.vortex-stacked-table { margin-top: -1.5px !important; }
      .vortex-data-row { border-bottom: 1px solid ${borderHex} !important; }
      .vortex-data-cell { border: 1px solid ${borderHex} !important; padding: 9px 12px !important; vertical-align: middle !important; }
      .vortex-data-head-cell, th { background-color: ${surfaceAltHex} !important; border: 1px solid ${borderHex} !important; border-bottom: 1.5px solid ${borderStrongHex} !important; padding: 9px 12px !important; }
      hr { border: 0 !important; border-top: 1.5px solid ${borderHex} !important; margin: 12px 0 !important; opacity: 1 !important; }
      blockquote { border-left: 3px solid ${borderStrongHex} !important; margin: 8px 0 !important; padding-left: 12px !important; }
      body * { font-family: "Vortex SF Pro", -apple-system, system-ui, "Segoe UI", Roboto, sans-serif !important; font-weight: 700 !important; }
      pre, code { white-space: pre-wrap !important; overflow-x: hidden !important; word-break: break-word; }
    </style></head><body>${bodyContent}</body></html>`;
  }

  function selectItem(id, openMobile = true) {
    const changed = selectedId !== id;
    selectedId = id;
    updateStreamSelectionAttr();
    const item = items.find((i) => i.id === id);
    if (!item) return;
    if (openMobile) {
      $("inboxGrid").classList.add("has-mobile-reader");
    }
    $("readerEmptyState").classList.add("hidden");
    const wrap = $("readerContentWrap");
    wrap.classList.remove("hidden");

    const htmlLen = (item.body_html || "").length;
    const textLen = (item.body_text || "").length;
    if (!changed && lastRenderedItemId === id && lastRenderedHtmlLen === htmlLen + textLen) {
      return;
    }
    lastRenderedItemId = id;
    lastRenderedHtmlLen = htmlLen + textLen;

    const now = performance.now();
    const canAnimateReader = changed && (now - lastReaderAnimAt >= ANIM_COOLDOWN_MS);
    if (canAnimateReader) {
      lastReaderAnimAt = now;
      wrap.classList.remove("reader-animate");
      void wrap.offsetWidth;
      wrap.classList.add("reader-animate");
    }

    $("readerSubject").textContent = item.subject || "(Không tiêu đề)";
    const metaEl = $("readerMeta");
    metaEl.innerHTML = "";
    const avatar = createSenderAvatar(item.sender);

    const stack = document.createElement("div");
    stack.className = "reader-sender-stack";

    const strongName = document.createElement("span");
    strongName.className = "reader-sender-strong";
    strongName.textContent = cleanSenderName(item.sender);

    const subLine = document.createElement("span");
    subLine.className = "reader-sender-sub";
    const emailPart = cleanSenderEmail(item.sender);
    const timeStr = new Date(item.received_at * 1000).toLocaleString();
    const accTag = item.account ? ` · ${item.account}` : "";
    subLine.textContent = (emailPart ? `${emailPart} · ` : "") + `${catLabel(item.category)}${accTag} · ${timeStr}`;

    stack.append(strongName, subLine);
    metaEl.append(avatar, stack);

    const phish = $("readerPhishing");
    if (item.phishing_warning) {
      $("readerPhishingText").textContent = item.phishing_warning;
      phish.classList.remove("hidden");
    } else {
      phish.classList.add("hidden");
    }
    const unsubBtn = $("btnUnsubscribe");
    if (item.unsubscribe_link) {
      unsubBtn.classList.remove("hidden");
      unsubBtn.onclick = () => window.open(item.unsubscribe_link, "_blank", "noopener,noreferrer");
    } else {
      unsubBtn.classList.add("hidden");
    }

    const bodyTextClean = cleanBodyText(item.subject, item.body_text || "");
    const linkBtn = $("btnOpenBodyLink");
    const firstUrlMatch = bodyTextClean.match(/https?:\/\/[^\s"'<>)]+/);
    const actionLink = (item.extracted_links && item.extracted_links[0] && item.extracted_links[0].url) || (firstUrlMatch && firstUrlMatch[0]);
    if (actionLink && !item.phishing_warning) {
      linkBtn.classList.remove("hidden");
      linkBtn.onclick = () => window.open(actionLink, "_blank", "noopener,noreferrer");
    } else {
      linkBtn.classList.add("hidden");
    }

    const ent = $("readerEntities");
    if (item.extracted_entities && item.extracted_entities.length) {
      ent.textContent = item.extracted_entities.map((e) => `${e.label}: ${e.value}`).join(" | ");
    } else {
      ent.textContent = "";
    }

    const frame = $("readerHtmlFrame");
    const bodyDiv = $("readerBody");
    frame.classList.remove("hidden");
    bodyDiv.classList.add("hidden");
    frame.srcdoc = buildEmailIframeDoc(item.body_html, bodyTextClean, canAnimateReader);

    if ((!item.body_html || !/<(table|p|br|a\s)/i.test(item.body_html)) && (item.body_text || "").length < 300) {
      triggerSync(item.account || null, true);
    }
  }

  async function runAction(itemId, actionObj) {
    if (!itemId) return;
    const targetItem = items.find((i) => i.id === itemId) || cachedAllItems.find((i) => i.id === itemId);
    let effectiveAction = actionObj.action;
    if (effectiveAction === "pin" && targetItem && targetItem.pinned) {
      effectiveAction = "unpin";
    }
    if (effectiveAction === "delete" || effectiveAction === "archive") {
      playCreamySfx("delete");
      if (targetItem) markItemHidden(targetItem);
      const prevIdx = items.findIndex((i) => i.id === itemId);
      items = items.filter((i) => i.id !== itemId);
      cachedAllItems = cachedAllItems.filter((i) => i.id !== itemId);
      recentCodes = recentCodes.filter((c) => c.item_id !== itemId);
      updateCategoryCounts(cachedAllItems);
      renderAccountSwitcher(cachedAllItems);
      if (items.length > 0) {
        const nextIdx = Math.min(Math.max(0, prevIdx), items.length - 1);
        selectItem(items[nextIdx].id, false);
      } else {
        selectedId = null;
        lastRenderedItemId = null;
        $("readerContentWrap").classList.add("hidden");
        $("readerEmptyState").classList.remove("hidden");
      }
      renderStream(true);
      renderRecentCodes();
    }
    await api("/api/actions", {
      method: "POST",
      body: JSON.stringify({ item_id: itemId, ...actionObj, action: effectiveAction })
    });
    const labels = {
      archive: lang === "vi" ? "Đã lưu trữ thư" : "Archived message",
      mark_read: lang === "vi" ? "Đã đánh dấu đã đọc" : "Marked as read",
      pin: lang === "vi" ? "Đã ghim thư" : "Pinned message",
      unpin: lang === "vi" ? "Đã bỏ ghim thư" : "Unpinned message",
      delete: lang === "vi" ? "Đã xóa thư" : "Deleted message"
    };
    if (labels[effectiveAction]) showToast(labels[effectiveAction]);
    await loadAll();
  }

  $("btnMobileBack").onclick = () => {
    $("inboxGrid").classList.remove("has-mobile-reader");
  };

  $("btnModeSingle").onclick = () => setAddMode("single");
  $("btnModeBulk").onclick = () => setAddMode("bulk");

  document.querySelectorAll(".provider-grid .preset-btn").forEach((btn) => {
    btn.onclick = () => selectPreset(btn.getAttribute("data-preset") || "gmail");
  });

  $("btnTogglePw").onclick = () => {
    const inp = $("accSecret");
    const isPw = inp.type === "password";
    inp.type = isPw ? "text" : "password";
    $("btnTogglePw").textContent = isPw ? t("hide_pw") : t("show_pw");
  };

  let googlePollTimer = null;
  let lastKnownAccCount = -1;
  let lastKnownItemsCount = -1;

  $("btnGoogleLogin").onclick = async () => {
    $("setupErrorBox").classList.add("hidden");
    const email = $("accEmail").value.trim();
    const prevIds = new Set(accounts.map((a) => a.id));
    $("btnGoogleLoginText").textContent = lang === "vi"
      ? "Đang mở cửa sổ Đăng nhập Google..."
      : "Opening Google Sign-In window...";
    await api("/api/google-login", {
      method: "POST",
      body: JSON.stringify({ email })
    });
    if (googlePollTimer) clearInterval(googlePollTimer);
    let pollTicks = 0;
    googlePollTimer = setInterval(async () => {
      pollTicks += 1;
      try {
        const accs = await api("/api/accounts");
        const added = accs.find((a) => !prevIds.has(a.id));
        if (added || (email && accs.some((a) => a.id === email) && pollTicks >= 4)) {
          clearInterval(googlePollTimer);
          googlePollTimer = null;
          $("btnGoogleLoginText").textContent = t("google_login_btn");
          activeAccount = added ? added.id : email;
          setScreenMode("inbox");
          await loadAll();
        }
      } catch (_) {}
    }, 450);
  };

  $("accountForm").onsubmit = async (e) => {
    e.preventDefault();
    $("setupErrorBox").classList.add("hidden");
    const submitBtn = $("btnSubmitAccount");
    const submitTxt = $("btnSubmitAccountText");
    submitBtn.disabled = true;
    submitTxt.textContent = lang === "vi" ? "Đang kết nối..." : "Connecting...";

    const source = "email";
    const email = $("accEmail").value.trim();
    const host = $("accHost").value.trim();
    const port = Number($("accPort").value) || 993;
    const secret = $("accSecret").value;
    try {
      await api("/api/accounts", {
        method: "POST",
        body: JSON.stringify({
          id: email,
          source,
          email_or_handle: email,
          host,
          port,
          use_tls: port === 993 || port === 443,
          secret,
          async_sync: true
        })
      });
      $("accEmail").value = "";
      $("accSecret").value = "";
      activeAccount = email;
      setScreenMode("inbox");
      $("statusNotice").textContent = lang === "vi"
        ? "Đang đồng bộ hộp thư mới..."
        : "Syncing new mailbox...";
      await loadAll();
    } catch (err) {
      const msg = String(err.message || err);
      $("setupErrorText").textContent = lang === "vi"
        ? `Không thể đăng nhập: ${msg}. Hãy kiểm tra lại Mật khẩu ứng dụng 16 ký tự.`
        : `Login failed: ${msg}. Please check your App Password.`;
      $("setupErrorBox").classList.remove("hidden");
    } finally {
      submitBtn.disabled = false;
      submitTxt.textContent = t("save_connect");
    }
  };

  $("bulkAccountForm").onsubmit = async (e) => {
    e.preventDefault();
    $("setupErrorBox").classList.add("hidden");
    const rawText = $("bulkAccountsInput").value.trim();
    if (!rawText) return;
    const btn = $("btnSubmitBulk");
    const txt = $("btnSubmitBulkText");
    btn.disabled = true;
    txt.textContent = lang === "vi"
      ? "Đang thêm tài khoản..."
      : "Adding accounts...";
    try {
      const res = await api("/api/accounts/batch", {
        method: "POST",
        body: JSON.stringify({
          text: rawText,
          default_host: $("accHost").value.trim() || "imap.gmail.com",
          default_port: Number($("accPort").value) || 993,
          async_sync: true
        })
      });
      $("bulkAccountsInput").value = "";
      activeAccount = "";
      setScreenMode("inbox");
      showToast(
        lang === "vi"
          ? `Đã thêm ${res.added || 0} tài khoản, đang đồng bộ ngầm...`
          : `Added ${res.added || 0} accounts, syncing in background...`
      );
      await loadAll();
    } catch (err) {
      $("setupErrorText").textContent = String(err.message || err);
      $("setupErrorBox").classList.remove("hidden");
    } finally {
      btn.disabled = false;
      txt.textContent = t("bulk_submit");
    }
  };

  $("btnAcctMenu").onclick = (e) => {
    e.stopPropagation();
    const menu = $("acctMenu");
    if (!menu) return;
    const isHidden = menu.classList.contains("hidden");
    menu.classList.toggle("hidden", !isHidden);
    $("btnAcctMenu").setAttribute("aria-expanded", isHidden ? "true" : "false");
  };

  document.addEventListener("click", (e) => {
    const switcher = $("acctSwitcher");
    if (switcher && !switcher.contains(e.target)) {
      closeAccountMenu();
    }
  });

  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape") {
      closeAccountMenu();
    }
  });

  $("btnToggleSetup").onclick = () => {
    closeAccountMenu();
    const isSetup = !$("onboardingView").classList.contains("hidden");
    if (isSetup && accounts.length > 0) {
      setScreenMode("inbox");
    } else {
      setScreenMode("setup");
    }
  };
  $("btnCloseSetup").onclick = () => setScreenMode("inbox");

  $("btnSync").onclick = () => triggerSync(activeAccount || null, false);

  const postWinIpc = (msg) => {
    try {
      if (window.ipc && typeof window.ipc.postMessage === "function") {
        window.ipc.postMessage(msg);
      }
    } catch (_) {}
  };

  if ($("btnWinMin")) $("btnWinMin").onclick = () => postWinIpc("win:min");
  if ($("btnWinMax")) $("btnWinMax").onclick = () => postWinIpc("win:max");
  if ($("btnWinClose")) $("btnWinClose").onclick = () => postWinIpc("win:close");

  const topbarEl = document.querySelector(".topbar");
  if (topbarEl) {
    topbarEl.addEventListener("mousedown", (e) => {
      if (e.button !== 0) return;
      if (e.target.closest("button, input, select, textarea, a, #acctMenu")) return;
      postWinIpc("win:drag");
    });
    topbarEl.addEventListener("dblclick", (e) => {
      if (e.target.closest("button, input, select, textarea, a, #acctMenu")) return;
      postWinIpc("win:max");
    });
  }

  $("btnSfx").onclick = () => {
    sfxEnabled = !sfxEnabled;
    try {
      localStorage.setItem("vortex_sfx", sfxEnabled ? "1" : "0");
    } catch (_) {}
    applySfxButton();
    if (sfxEnabled) {
      playCreamySfx("copy");
    }
  };

  $("btnTheme").onclick = (e) => {
    const btn = $("btnTheme");
    if (btn) {
      btn.classList.remove("is-flipping");
      void btn.offsetWidth;
      btn.classList.add("is-flipping");
      setTimeout(() => btn.classList.remove("is-flipping"), 380);
    }

    const nextTheme = theme === "dark" ? "light" : "dark";
    const commitTheme = () => {
      theme = nextTheme;
      try {
        localStorage.setItem("vortex_theme", theme);
      } catch (_) {}
      applyTheme(true);
    };

    if (document.startViewTransition && !window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      const rect = btn ? btn.getBoundingClientRect() : null;
      const x = rect ? rect.left + rect.width / 2 : (e.clientX || window.innerWidth / 2);
      const y = rect ? rect.top + rect.height / 2 : (e.clientY || 0);
      const endRadius = Math.hypot(
        Math.max(x, window.innerWidth - x),
        Math.max(y, window.innerHeight - y)
      );
      const transition = document.startViewTransition(() => {
        commitTheme();
      });
      transition.ready.then(() => {
        document.documentElement.animate(
          {
            clipPath: [
              `circle(0px at ${x}px ${y}px)`,
              `circle(${endRadius}px at ${x}px ${y}px)`
            ]
          },
          {
            duration: 420,
            easing: "cubic-bezier(0.22, 1, 0.36, 1)",
            pseudoElement: "::view-transition-new(root)"
          }
        );
      }).catch(() => {});
    } else {
      document.documentElement.classList.add("theme-transitioning");
      commitTheme();
      setTimeout(() => {
        document.documentElement.classList.remove("theme-transitioning");
      }, 280);
    }
  };

  document.addEventListener("pointerdown", (e) => {
    const btn = e.target.closest("button");
    if (!btn || btn.disabled || btn.id === "btnSfx") return;
    if (btn.classList.contains("cat-btn") || btn.id === "btnAutoSort") {
      playCreamySfx("pill");
    } else if (btn.classList.contains("otp-copy") || btn.id === "btnCopyAllOtp") {
      playCreamySfx("copy");
    } else if (btn.getAttribute("data-act") === "delete" || btn.getAttribute("data-act") === "archive" || btn.classList.contains("acct-item-del")) {
      playCreamySfx("delete");
    } else {
      playCreamySfx("click");
    }
  });

  let langSwitchBusy = false;

  $("btnLang").onclick = () => {
    if (langSwitchBusy) return;
    langSwitchBusy = true;

    const btn = $("btnLang");
    if (btn) {
      btn.classList.remove("is-switching");
      void btn.offsetWidth;
      btn.classList.add("is-switching");
      setTimeout(() => btn.classList.remove("is-switching"), 420);
    }

    const targets = Array.from(
      document.querySelectorAll(
        "[data-i18n], #themeLabel, #acctActiveLabel, .stream-row .row-cat, .stream-row .row-time, .detail-meta-pill"
      )
    ).filter((el) => el.offsetParent !== null);

    targets.forEach((el, idx) => {
      const delay = Math.min(idx * 10, 90);
      el.style.setProperty("--i18n-delay", `${delay}ms`);
      el.classList.remove("i18n-wave-in");
      el.classList.add("i18n-wave-out");
    });

    setTimeout(() => {
      lang = lang === "vi" ? "en" : "vi";
      applyTranslations();

      const freshTargets = Array.from(
        document.querySelectorAll(
          "[data-i18n], #themeLabel, #acctActiveLabel, .stream-row .row-cat, .stream-row .row-time, .detail-meta-pill"
        )
      ).filter((el) => el.offsetParent !== null);

      freshTargets.forEach((el, idx) => {
        const delay = Math.min(idx * 12, 110);
        el.style.setProperty("--i18n-delay", `${delay}ms`);
        el.classList.remove("i18n-wave-out");
        el.classList.add("i18n-wave-in");
      });

      setTimeout(() => {
        freshTargets.forEach((el) => {
          el.classList.remove("i18n-wave-in");
          el.style.removeProperty("--i18n-delay");
        });
        langSwitchBusy = false;
      }, 380);
    }, 135);
  };

  $("searchInput").oninput = () => {
    clearTimeout(searchTimer);
    searchTimer = setTimeout(async () => {
      const q = $("searchInput").value.trim();
      if (!q) {
        await loadAll();
        return;
      }
      const accParam = activeAccount ? `&account=${encodeURIComponent(activeAccount)}` : "";
      const searched = await api(`/api/items?q=${encodeURIComponent(q)}${accParam}`);
      items = sortItemsList(searched.filter(isItemVisible));
      renderStream();
    }, 180);
  };

  const catButtons = Array.from(document.querySelectorAll(".category-pills .cat-btn"));
  catButtons.forEach((btn, idx) => {
    btn.onclick = () => {
      const nextCat = btn.getAttribute("data-cat") || "";
      if (nextCat === activeCategory) return;
      prevCatIndex = idx;
      catButtons.forEach((b) => b.setAttribute("aria-pressed", "false"));
      btn.setAttribute("aria-pressed", "true");
      updateCategoryGlider();
      activeCategory = nextCat;
      loadAll(true);
    };
  });

  $("btnAutoSort").onclick = () => {
    const now = performance.now();
    if (now - lastStreamMorphAt < 220) return;
    autoSortEnabled = !autoSortEnabled;
    $("btnAutoSort").setAttribute("aria-pressed", autoSortEnabled ? "true" : "false");
    const prevTopId = items.length > 0 ? items[0].id : null;
    items = sortItemsList(items);
    const nextTopId = items.length > 0 ? items[0].id : null;
    if (prevTopId !== nextTopId) {
      triggerStreamMorph(1);
    }
    renderStream(prevTopId !== nextTopId);
    if (items.length > 0 && window.innerWidth > 720 && selectedId !== items[0].id) {
      selectItem(items[0].id, false);
    }
    showToast(
      autoSortEnabled
        ? (lang === "vi" ? "Đã tự động sắp xếp: Ưu tiên Mã OTP và thư chưa đọc lên đầu" : "Auto-sort enabled: OTP codes and unread messages first")
        : (lang === "vi" ? "Đã sắp xếp theo thời gian nhận mới nhất" : "Sorted by newest received time")
    );
  };

  window.addEventListener("resize", () => {
    requestAnimationFrame(() => {
      updateCategoryGlider();
      const activeBtn = document.querySelector('.category-pills .cat-btn[aria-pressed="true"]');
      if (activeBtn) activeBtn.scrollIntoView({ block: "nearest", inline: "nearest" });
    });
  });

  document.querySelectorAll("[data-act]").forEach((btn) => {
    btn.onclick = async () => {
      if (!selectedId) return;
      const act = btn.getAttribute("data-act");
      await runAction(selectedId, { action: act });
    };
  });

  setInterval(async () => {
    try {
      const st = await api("/api/status");
      const accChanged = lastKnownAccCount !== -1 && st.accounts_count !== lastKnownAccCount;
      const itemsChanged = lastKnownItemsCount !== -1 && st.items_count !== lastKnownItemsCount;
      lastKnownAccCount = st.accounts_count;
      lastKnownItemsCount = st.items_count;

      if (!$("onboardingView").classList.contains("hidden")) {
        if (st.accounts_count > 0 && (!st.missing_secret_accounts || st.missing_secret_accounts.length === 0) && accounts.length === 0) {
          setScreenMode("inbox");
          await loadAll();
          return;
        }
      }
      if (accChanged || itemsChanged) {
        await loadAll();
        return;
      }
      recentCodes = await api("/api/codes");
      } catch (_) {}
  }, 900);

  setInterval(() => {
    if (accounts.length > 0 && !document.hidden) {
      triggerSync(null, true);
    }
  }, 10000);

  let knownUiMtime = 0;
  setInterval(async () => {
    try {
      const r = await api("/api/ui-mtime");
      if (r && r.mtime) {
        if (knownUiMtime && r.mtime !== knownUiMtime) {
          location.reload();
          return;
        }
        knownUiMtime = r.mtime;
      }
    } catch (_) {}
  }, 900);

  applyTheme(false);
  selectPreset("gmail");
  loadAll();
})();
"##;

pub const OTP_OVERLAY_HTML: &str = include_str!("otp_overlay.html");

pub const fn bundle_size_bytes() -> u64 {
    (INDEX_HTML.len() + STYLES_CSS.len() + APP_JS.len() + OTP_OVERLAY_HTML.len()) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use vortex_core::WEB_UI_MAX_ASSET_BYTES;

    #[test]
    fn bundle_stays_within_150kb_budget_and_hallmark_floor() {
        assert!(bundle_size_bytes() <= WEB_UI_MAX_ASSET_BYTES);
        assert!(STYLES_CSS.starts_with("/* hallmark"));
        assert!(!STYLES_CSS.contains("transition: all"));
        assert!(!STYLES_CSS.contains("linear-gradient"));
        assert!(!STYLES_CSS.contains("radial-gradient"));
        assert!(!STYLES_CSS.contains("100vw"));
        assert!(!STYLES_CSS.to_ascii_lowercase().contains("#000"));
        assert!(!STYLES_CSS.to_ascii_lowercase().contains("#fff"));
        assert!(!APP_JS.to_ascii_lowercase().contains("#000"));
        assert!(!APP_JS.to_ascii_lowercase().contains("#fff"));
        assert!(STYLES_CSS.contains("overflow-x: clip"));
        assert!(STYLES_CSS.contains("prefers-reduced-motion: reduce"));
        assert!(INDEX_HTML.contains("viewBox=\"0 0 20 20\" fill=\"currentColor\""));
        assert!(!INDEX_HTML.contains("fill=\"none\""));
    }
}
