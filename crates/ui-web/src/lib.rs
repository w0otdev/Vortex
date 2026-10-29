pub const STYLES_CSS: &str = r##"/* hallmark · v2.0.0 · shape: app screen · genre: modern-minimal · theme: paper-ink (custom) · signature: live-otp-banner */
/* critique: calm two-pane casual inbox with centered single-purpose onboarding card and Heroicons 20/solid glyphs */
/* waive: custom-app-workbench */
@font-face{
font-family:"Vortex SF Pro";
src:local("SF Pro Display Bold"),local("SFProDisplay-Bold");
font-style:normal;
font-weight:700;
font-display:swap;
}
:root{
--bg-page:#e4e4e0;--bg-surface:#f4f4f0;--bg-elevated:#fafaf7;--bg-subtle:#d6d6d0;
--bg-inverse:#1a1a18;--bg-alert:#f2dede;--bg-notice:#e6e6e1;--bg-backdrop:rgba(24,24,22,0.55);
--text-primary:#181816;--text-secondary:#383835;--text-muted:#5c5c57;--text-inverse:#f4f4f0;
--text-danger:#8f2424;--text-accent:#181816;--border-subtle:#94948b;--border-strong:#1a1a18;--border-danger:#b83c3c;
--font-sans:"Vortex SF Pro",-apple-system,system-ui,"Segoe UI",Roboto,sans-serif;--font-mono:var(--font-sans);
--radius-base:10px;--container-max:1280px;--col-gap:16px;
--glass-surface:#f4f4f0;--glass-edge:#9e9e96;--bevel-top:#fcfcf9;--bevel-bottom:#dcdcd4;
--pill-ledge:#8c8c84;--pill-ledge-strong:#0e0e0d;--shadow-ambient:rgba(24,24,22,0.12);
--glass-shadow:inset 0 1.5px 0 var(--bevel-top),inset 0 -1px 0 var(--bevel-bottom),0 3px 0 var(--pill-ledge),0 7px 14px var(--shadow-ambient);
--btn-3d-shadow:inset 0 1.5px 0 var(--bevel-top),inset 0 -1px 0 var(--bevel-bottom),0 3px 0 var(--pill-ledge),0 4px 8px var(--shadow-ambient);
--btn-3d-hover-shadow:inset 0 1.5px 0 var(--bevel-top),inset 0 -1px 0 var(--bevel-bottom),0 4px 0 var(--pill-ledge),0 7px 12px var(--shadow-ambient);
--btn-3d-active-shadow:inset 0 1px 0 var(--bevel-bottom),0 1px 0 var(--pill-ledge),0 2px 4px var(--shadow-ambient);
--btn-primary-shadow:inset 0 1.5px 0 rgba(244,244,240,0.22),inset 0 -1px 0 rgba(14,14,13,0.5),0 3px 0 var(--pill-ledge-strong),0 5px 10px var(--shadow-ambient);
--btn-primary-active-shadow:inset 0 1px 0 rgba(14,14,13,0.6),0 1px 0 var(--pill-ledge-strong);
--glider-surface:#1c1c1a;--glider-text:#f4f4f0;--glider-edge:#121210;
--glider-shadow:inset 0 1.5px 0 rgba(244,244,240,0.24),inset 0 -1px 0 rgba(14,14,13,0.6),0 2.5px 0 #0e0e0d,0 5px 10px rgba(24,24,22,0.18);
--glass-hover:rgba(24,24,22,0.07);--glass-selected:#dfdfd9;
--row-border:#b0b0a6;--row-hover:#eaeae4;--row-selected:#dcdcd4;
--scrollbar-thumb:#9e9e95;--scrollbar-thumb-hover:#62625b;--scrollbar-thumb-active:#1a1a18;
}
[data-theme="dark"]{
--bg-page:#121211;--bg-surface:#1b1b19;--bg-elevated:#232320;--bg-subtle:#282825;
--bg-inverse:#ecece6;--bg-alert:#2e1b1b;--bg-notice:#242421;--bg-backdrop:rgba(12,12,11,0.72);
--text-primary:#ecece6;--text-secondary:#bdbdb6;--text-muted:#8a8a82;--text-inverse:#161614;
--text-danger:#e07070;--text-accent:#ecece6;--border-subtle:#4a4a44;--border-strong:#ecece6;--border-danger:#b84a4a;
--glass-surface:#20201e;--glass-edge:#3e3e39;--bevel-top:rgba(236,236,230,0.14);--bevel-bottom:rgba(11,11,10,0.65);
--pill-ledge:#0c0c0b;--pill-ledge-strong:#8e8e86;--shadow-ambient:rgba(10,10,9,0.55);
--glass-shadow:inset 0 1.5px 0 var(--bevel-top),inset 0 -1px 0 var(--bevel-bottom),0 3px 0 var(--pill-ledge),0 8px 16px var(--shadow-ambient);
--btn-3d-shadow:inset 0 1.5px 0 var(--bevel-top),inset 0 -1px 0 var(--bevel-bottom),0 3px 0 var(--pill-ledge),0 5px 10px var(--shadow-ambient);
--btn-3d-hover-shadow:inset 0 1.5px 0 var(--bevel-top),inset 0 -1px 0 var(--bevel-bottom),0 4px 0 var(--pill-ledge),0 8px 14px var(--shadow-ambient);
--btn-3d-active-shadow:inset 0 1px 0 var(--bevel-bottom),0 1px 0 var(--pill-ledge),0 2px 4px var(--shadow-ambient);
--btn-primary-shadow:inset 0 1.5px 0 #fafaf5,inset 0 -1px 0 #c8c8c0,0 3px 0 var(--pill-ledge-strong),0 6px 12px var(--shadow-ambient);
--btn-primary-active-shadow:inset 0 1px 0 #b8b8b0,0 1px 0 var(--pill-ledge-strong);
--glider-surface:#ecece6;--glider-text:#161614;--glider-edge:#f4f4ee;
--glider-shadow:inset 0 1.5px 0 #fafaf5,inset 0 -1px 0 #c6c6be,0 2.5px 0 #8a8a82,0 5px 12px rgba(10,10,9,0.55);
--glass-hover:rgba(236,236,230,0.08);--glass-selected:#2f2f2b;
--row-border:#3a3a35;--row-hover:#232320;--row-selected:#2d2d29;
--scrollbar-thumb:#4e4e48;--scrollbar-thumb-hover:#7c7c74;--scrollbar-thumb-active:#ecece6;
}
*,*::before,*::after{box-sizing:border-box}
*::-webkit-scrollbar{width:8px;height:0 !important;background:transparent}
*::-webkit-scrollbar:horizontal,*::-webkit-scrollbar-button,*::-webkit-scrollbar-corner{display:none !important;width:0 !important;height:0 !important;background:transparent}
*::-webkit-scrollbar-track{background:transparent;margin-block:6px}
*::-webkit-scrollbar-thumb{background-color:var(--scrollbar-thumb);border-radius:999px;border:2px solid transparent;background-clip:padding-box;min-height:36px}
*::-webkit-scrollbar-thumb:hover{background-color:var(--scrollbar-thumb-hover)}
*::-webkit-scrollbar-thumb:active{background-color:var(--scrollbar-thumb-active)}
html,body{margin:0;padding:0;width:100%;height:100%;min-height:100vh;overflow-x: clip;overflow-y:hidden;background-color:var(--bg-page);color:var(--text-primary);font-family:var(--font-sans);font-weight:700;font-size:16px;line-height:1.5}
h1,h2,h3{margin:0;font-weight:700;color:var(--text-primary);overflow-wrap:break-word;min-width:0;letter-spacing:0}
p{margin:0}
button,input,select,textarea{font-family:var(--font-sans);font-weight:700;font-size:0.9375rem;color:var(--text-primary)}
:focus-visible{outline:2px solid var(--text-accent);outline-offset:2px}
.svg-sprite{position:absolute;width:0;height:0;overflow:hidden;pointer-events:none}
.hi-icon{width:18px;height:18px;flex-shrink:0;fill:currentColor;display:inline-block}
.hi-icon-lg{width:22px;height:22px;flex-shrink:0;fill:currentColor}
@keyframes spin{to{transform:rotate(360deg)} }
@keyframes rowFadeUp{0%{opacity:0;transform:translate3d(0,8px,0)} 100%{opacity:1;transform:translate3d(0,0,0)} }
@keyframes readerStageIn{0%{opacity:0;transform:translate3d(0,12px,0)} 100%{opacity:1;transform:translate3d(0,0,0)} }
@keyframes readerCanvasUnfold{0%{opacity:0;transform:translate3d(0,16px,0)} 100%{opacity:1;transform:translate3d(0,0,0)} }
@keyframes panelEnter{from{opacity:0;transform:translateY(12px)} to{opacity:1;transform:translateY(0)} }
.is-spinning .hi-icon{animation:spin 700ms linear infinite}
@keyframes syncPop{0%{transform:scale(0.55) rotate(-18deg);opacity:0} 55%{transform:scale(1.18) rotate(0deg);opacity:1} 100%{transform:scale(1) rotate(0deg);opacity:1} }
.topbar .refresh-btn.is-synced{background:var(--bg-inverse);color:var(--text-inverse);border-color:var(--border-strong);box-shadow:var(--btn-primary-shadow)}
.topbar .refresh-btn.is-synced .hi-icon{animation:syncPop 380ms cubic-bezier(0.22,1,0.36,1) forwards}
.shell{max-width:var(--container-max);margin-inline:auto;height:100vh;display:flex;flex-direction:column;overflow:hidden;padding:14px 20px 14px}
.topbar{position:relative;z-index:10;flex-shrink:0;display:grid;grid-template-columns:auto minmax(200px,1fr) auto auto auto;align-items:center;gap:12px;padding:4px 0 12px;-webkit-user-select:none;user-select:none}
.window-controls{display:inline-flex;align-items:center;gap:4px;padding:5px}
.topbar .win-btn{width:38px;min-width:38px;padding:0;justify-content:center}
.topbar .win-btn-close:hover{background:var(--bg-alert);color:var(--text-danger);border-color:var(--border-danger)}
.glass-pill{background:var(--glass-surface);border:1px solid var(--glass-edge);border-radius:999px;box-shadow:var(--glass-shadow);-webkit-backdrop-filter:blur(24px) saturate(140%);backdrop-filter:blur(24px) saturate(140%);transition:transform 160ms cubic-bezier(0.22,1,0.36,1),box-shadow 160ms ease,background-color 160ms ease,border-color 160ms ease}
.brand-mark{display:inline-flex;align-items:center;gap:10px;color:var(--text-accent);white-space:nowrap;flex-shrink:0}
.brand-mark > .hi-icon-lg{width:28px;height:28px;color:var(--text-primary)}
.brand-title{font-size:1.125rem;letter-spacing:-0.035em}
.search-wrap{position:relative;display:flex;align-items:center;min-width:0;padding:4px}
.search-wrap > .hi-icon{position:absolute;left:16px;top:50%;transform:translateY(-50%);color:var(--text-muted);pointer-events:none;transition:color 150ms ease}
.search-wrap:focus-within > .hi-icon{color:var(--text-accent)}
.input-field{width:100%;min-height:40px;padding:8px 14px;background-color:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-base);box-shadow:inset 0 2px 4px var(--shadow-ambient);transition:border-color 150ms ease,box-shadow 150ms ease}
.input-field:focus{border-color:var(--text-accent);box-shadow:0 0 0 3px var(--glass-hover);outline:none}
.search-wrap .input-field{min-width:0;min-height:40px;padding:8px 12px 8px 40px;background:transparent;border:0;border-radius:999px;font-size:0.875rem;box-shadow:none}
.search-wrap:focus-within{outline:2px solid var(--text-accent);outline-offset:2px}
.mailbox-controls{display:flex;align-items:center;min-width:0;gap:6px;padding:5px}
.topbar .btn{min-height:38px;padding-inline:13px;border:1px solid transparent;border-radius:999px;background:transparent;box-shadow:none;font-size:0.8125rem;font-weight:700}
.topbar .btn:hover{background:var(--glass-selected);border-color:var(--glass-edge);box-shadow:inset 0 1px 0 var(--bevel-top),0 2px 0 var(--pill-ledge);transform:translateY(-1px)}
.topbar .btn:active{transform:translateY(1px);box-shadow:inset 0 1px 2px var(--shadow-ambient)}
.topbar .refresh-btn{flex-shrink:0;width:38px;padding:0;color:var(--text-accent);background:var(--bg-elevated);border:1px solid var(--glass-edge);box-shadow:inset 0 1px 0 var(--bevel-top),0 2px 0 var(--pill-ledge)}
.topbar .refresh-btn:hover{background:var(--glass-selected);transform:translateY(-1px);box-shadow:inset 0 1px 0 var(--bevel-top),0 3px 0 var(--pill-ledge)}
.topbar .refresh-btn:active{transform:translateY(1.5px);box-shadow:0 0.5px 0 var(--pill-ledge)}
.topbar .btn-primary{background:var(--bg-inverse);color:var(--text-inverse);border:1px solid var(--border-strong);box-shadow:var(--btn-primary-shadow)}
.topbar .btn-primary:hover{background:var(--bg-inverse);border-color:var(--border-strong);box-shadow:var(--btn-primary-shadow);opacity:0.94;transform:translateY(-1px)}
.topbar .btn-primary:active{transform:translateY(2px);box-shadow:var(--btn-primary-active-shadow)}
.language-controls{display:inline-flex;align-items:center;gap:4px;padding:5px}
#btnTheme{min-width:82px;justify-content:center}
#btnTheme .hi-icon{transition:transform 360ms cubic-bezier(0.34,1.56,0.64,1)}
#btnTheme.is-flipping .hi-icon{transform:rotate(180deg) scale(1.15)}
::view-transition-old(root),::view-transition-new(root){animation:none;mix-blend-mode:normal}
::view-transition-new(root){z-index:9999}
html.theme-transitioning,html.theme-transitioning *,html.theme-transitioning *::before,html.theme-transitioning *::after{transition:background-color 260ms cubic-bezier(0.22,1,0.36,1),color 220ms cubic-bezier(0.22,1,0.36,1),border-color 260ms cubic-bezier(0.22,1,0.36,1),box-shadow 260ms cubic-bezier(0.22,1,0.36,1) !important}
#btnLang{min-width:66px;justify-content:center;perspective:260px}
#btnLang .hi-icon{transition:transform 440ms cubic-bezier(0.34,1.56,0.64,1);transform-style:preserve-3d}
#btnLang.is-switching .hi-icon{transform:rotateY(180deg) rotateZ(-10deg) scale(1.2)}
#langLabel{display:inline-block;transform-origin:50% 50%}
@keyframes langDrumRoll{0%{opacity:1;transform:translateY(0) rotateX(0deg) scale(1);filter:blur(0px)} 42%{opacity:0;transform:translateY(-9px) rotateX(75deg) scale(0.84);filter:blur(2.5px)} 43%{opacity:0;transform:translateY(9px) rotateX(-75deg) scale(0.84);filter:blur(2.5px)} 100%{opacity:1;transform:translateY(0) rotateX(0deg) scale(1);filter:blur(0px)} }
#btnLang.is-switching #langLabel{animation:langDrumRoll 380ms cubic-bezier(0.22,1,0.36,1) forwards}
@keyframes i18nWaveOut{0%{opacity:1;transform:translateY(0) scale(1);filter:blur(0px)} 100%{opacity:0;transform:translateY(-5px) scale(0.96);filter:blur(3px)} }
@keyframes i18nWaveIn{0%{opacity:0;transform:translateY(6px) scale(0.96);filter:blur(3px)} 100%{opacity:1;transform:translateY(0) scale(1);filter:blur(0px)} }
.i18n-wave-out{animation:i18nWaveOut 130ms cubic-bezier(0.4,0,1,1) forwards;animation-delay:var(--i18n-delay,0ms)}
.i18n-wave-in{animation:i18nWaveIn 260ms cubic-bezier(0.22,1,0.36,1) both;animation-delay:var(--i18n-delay,0ms)}
.topbar .acct-trigger{max-width:210px;background:var(--bg-elevated);border:1px solid var(--glass-edge);box-shadow:inset 0 1px 0 var(--bevel-top),0 2px 0 var(--pill-ledge)}
.acct-chevron{width:12px;height:12px;flex-shrink:0;transition:transform 180ms ease}
.acct-trigger[aria-expanded="true"] .acct-chevron{transform:rotate(180deg)}
.btn{appearance:none;display:inline-flex;align-items:center;justify-content:center;gap:8px;min-height:38px;padding:0 14px;background-color:var(--bg-elevated);color:var(--text-primary);border:1px solid var(--glass-edge);border-radius:999px;box-shadow:var(--btn-3d-shadow);white-space:nowrap;cursor:pointer;transition:transform 140ms cubic-bezier(0.22,1,0.36,1),box-shadow 140ms ease,background-color 140ms ease,border-color 140ms ease,opacity 140ms ease}
.btn:hover{background-color:var(--glass-selected);box-shadow:var(--btn-3d-hover-shadow);transform:translateY(-1.5px)}
.btn:active{transform:translateY(2px);box-shadow:var(--btn-3d-active-shadow)}
.btn:disabled{opacity:0.5;cursor:not-allowed}
.btn-primary{background-color:var(--bg-inverse);color:var(--text-inverse);border-color:var(--border-strong);box-shadow:var(--btn-primary-shadow);font-weight:700}
.btn-primary:hover{opacity:0.94;background-color:var(--bg-inverse);box-shadow:var(--btn-primary-shadow)}
.btn-primary:active{transform:translateY(2px);box-shadow:var(--btn-primary-active-shadow)}
.btn-large{width:100%;min-height:44px;font-size:1rem}
.btn-google{width:100%;min-height:46px;margin-top:18px;font-size:1rem;font-weight:700;background-color:var(--bg-inverse);color:var(--text-inverse);border-color:var(--border-strong);box-shadow:var(--btn-primary-shadow)}
.btn-google:hover{opacity:0.94;background-color:var(--bg-inverse);box-shadow:var(--btn-primary-shadow)}
.btn-google:active{transform:translateY(2px);box-shadow:var(--btn-primary-active-shadow)}
.divider-caption{margin-top:18px;padding-top:14px;border-top:1px solid var(--border-subtle);font-size:0.8125rem;color:var(--text-muted)}
.btn-danger{background-color:var(--bg-alert);color:var(--text-danger);border-color:var(--border-danger)}
.btn-ghost{background-color:transparent;border-color:transparent;box-shadow:none;color:var(--text-secondary)}
.btn-ghost:hover{background-color:var(--bg-subtle);color:var(--text-primary)}
.otp-meta{font-size:.8125rem;color:var(--text-secondary)}
.onboarding-wrap{width:100%;max-width:640px;max-height:calc(100vh - 96px);overflow-y:auto;margin:16px auto 0;background-color:var(--bg-surface);border:1px solid var(--glass-edge);border-radius:20px;box-shadow:var(--glass-shadow);padding:24px 28px;animation:panelEnter 240ms cubic-bezier(0.22,1,0.36,1)}
.onboarding-head{display:flex;align-items:flex-start;justify-content:space-between;gap:12px}
.onboarding-title{font-size:1.375rem}
.onboarding-sub{margin-top:6px;font-size:0.9375rem;color:var(--text-secondary)}
.mode-switch{margin-top:18px;display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:10px}
.provider-grid{margin-top:16px;display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:10px}
.preset-btn{appearance:none;display:flex;align-items:center;justify-content:center;gap:8px;min-height:42px;padding:0 14px;background-color:var(--bg-elevated);color:var(--text-primary);border:1px solid var(--glass-edge);border-radius:999px;box-shadow:var(--btn-3d-shadow);font-size:0.9375rem;font-weight:700;cursor:pointer;transition:transform 140ms cubic-bezier(0.22,1,0.36,1),box-shadow 140ms ease,background-color 140ms ease,color 140ms ease,border-color 140ms ease}
.preset-btn:hover{background-color:var(--glass-selected);box-shadow:var(--btn-3d-hover-shadow);transform:translateY(-1.5px)}
.preset-btn:active{transform:translateY(2px);box-shadow:var(--btn-3d-active-shadow)}
.preset-btn[aria-pressed="true"]{background-color:var(--bg-inverse);color:var(--text-inverse);border-color:var(--border-strong);box-shadow:var(--btn-primary-shadow);font-weight:700}
.preset-btn[aria-pressed="true"]:active{transform:translateY(2px);box-shadow:var(--btn-primary-active-shadow)}
.form-stack{margin-top:16px;display:flex;flex-direction:column;gap:14px}
.bulk-textarea{min-height:120px;font-family:var(--font-mono);font-size:0.875rem;line-height:1.5;resize:vertical}
.field-label{display:block;font-size:0.875rem;font-weight:700;color:var(--text-primary);margin-bottom:6px}
.field-hint{margin-top:6px;font-size:0.8125rem;color:var(--text-secondary)}
.secret-input-group{display:flex;gap:8px}
.server-grid{display:grid;grid-template-columns:2fr 1fr;gap:12px}
.acct-switcher{position:relative;display:inline-flex}
.acct-trigger{max-width:240px}
.acct-active-label{overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.acct-menu{position:absolute;top:calc(100% + 6px);right:0;width:min(340px,calc(100dvw - 40px));max-height:400px;overflow-y:auto;background-color:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:24px;padding:10px;z-index:850;box-shadow:0 12px 32px rgba(24,24,22,0.14);animation:panelEnter 180ms cubic-bezier(0.16,1,0.3,1)}
.acct-menu-list{display:flex;flex-direction:column;gap:2px}
.acct-item-wrap{display:flex;align-items:center;gap:4px;border-radius:6px;padding-right:4px;transition:background-color 140ms ease}
.acct-item-wrap:hover,.acct-item-wrap[data-active="true"]{background-color:var(--bg-subtle)}
.acct-item-btn{flex:1;min-width:0;display:flex;align-items:center;justify-content:space-between;gap:10px;padding:8px 10px;background:transparent;border:none;border-radius:6px;color:var(--text-primary);font-family:inherit;font-size:0.875rem;text-align:left;cursor:pointer}
.acct-item-wrap[data-active="true"] .acct-item-btn{font-weight:700}
.acct-item-main{display:flex;align-items:center;gap:8px;min-width:0}
.acct-item-text{display:flex;flex-direction:column;min-width:0}
.acct-item-email{overflow:hidden;text-overflow:ellipsis;white-space:nowrap;line-height:1.3}
.acct-item-sub{font-size:0.75rem;font-weight:700;color:var(--text-muted);line-height:1.3}
.acct-item-del{flex-shrink:0;width:28px;height:28px;display:inline-flex;align-items:center;justify-content:center;background:transparent;border:none;border-radius:6px;color:var(--text-muted);cursor:pointer;transition:color 140ms ease,background-color 140ms ease}
.acct-item-del:hover{color:var(--status-danger);background-color:var(--status-danger-bg)}
.acct-menu-foot{margin-top:6px;padding-top:6px;border-top:1px solid var(--border-subtle);display:flex;align-items:center;justify-content:space-between;gap:6px}
.account-row-actions{display:flex;flex-wrap:wrap;align-items:center;gap:6px}
.btn-sm{min-height:32px;padding:0 10px;font-size:0.8125rem}
#inboxView{flex:1 1 0%;min-height:0;display:flex;flex-direction:column;overflow:hidden}
@keyframes catIconSpring{0%{transform:scale(0.78) rotate(-8deg)} 55%{transform:scale(1.14) rotate(3deg)} 100%{transform:scale(1) rotate(0deg)} }
@keyframes catStreamMorph{0%{opacity:0;transform:translate3d(0,6px,0)} 100%{opacity:1;transform:translate3d(0,0,0)} }
.filter-bar{flex-shrink:0;margin-top:4px;display:flex;flex-wrap:nowrap;align-items:center;justify-content:space-between;gap:6px;padding:4px;width:100%;min-width:0}
.filter-bar-right{flex-shrink:0;display:flex;align-items:center;gap:4px;padding-left:6px;border-left:1px solid var(--glass-edge);min-width:0}
.filter-bar-right .sort-btn{border-radius:999px;border:1px solid var(--glass-edge);background:var(--bg-elevated);box-shadow:inset 0 1px 0 var(--bevel-top),0 1.5px 0 var(--pill-ledge);min-height:36px;padding-inline:12px;gap:5px;font-size:0.8125rem;font-weight:700}
.filter-bar-right .sort-btn:hover{background-color:var(--glass-selected);box-shadow:inset 0 1px 0 var(--bevel-top),0 2.5px 0 var(--pill-ledge);transform:translateY(-1px)}
.filter-bar-right .sort-btn:active{transform:translateY(1.5px);box-shadow:0 0.5px 0 var(--pill-ledge)}
#statusNotice{display:none !important}
.sort-btn[aria-pressed="true"]{background-color:var(--bg-inverse);color:var(--text-inverse);border-color:var(--border-strong);box-shadow:var(--btn-primary-shadow);font-weight:700}
.sort-btn[aria-pressed="true"]:hover{background-color:var(--bg-inverse);color:var(--text-inverse);box-shadow:var(--btn-primary-shadow)}
.sort-btn[aria-pressed="true"]:active{transform:translateY(1.5px);box-shadow:var(--btn-primary-active-shadow)}
.sort-btn[aria-pressed="true"] .hi-icon{color:var(--text-inverse);animation:catIconSpring 460ms cubic-bezier(0.16,1,0.3,1)}
.category-pills{position:relative;flex:1 1 0%;display:flex;flex-wrap:nowrap;align-items:center;gap:2px;max-width:100%;min-width:0;padding:1px;border-radius:999px;overflow-x:auto;overscroll-behavior-x:contain;scrollbar-width:none}
.category-pills::-webkit-scrollbar{display:none}
.cat-glider{position:absolute;top:0;left:0;width:0;height:0;background-color:var(--glider-surface);border:1px solid var(--glider-edge);border-radius:999px;box-shadow:var(--glider-shadow);pointer-events:none;z-index:0;opacity:0;transition:transform 420ms cubic-bezier(0.16,1,0.3,1),width 420ms cubic-bezier(0.16,1,0.3,1),height 420ms cubic-bezier(0.16,1,0.3,1),opacity 200ms ease}
.cat-btn{position:relative;z-index:1;appearance:none;display:inline-flex;align-items:center;justify-content:center;gap:5px;flex:1 0 auto;min-height:36px;padding:0 10px;background-color:transparent;color:var(--text-secondary);border:1px solid transparent;border-radius:999px;font-size:0.8125rem;font-weight:700;white-space:nowrap;cursor:pointer;transition:transform 160ms cubic-bezier(0.16,1,0.3,1),color 200ms ease,background-color 160ms ease,box-shadow 160ms ease,border-color 160ms ease}
.cat-btn .hi-icon,.sort-btn .hi-icon{width:16px;height:16px}
.cat-btn:hover{color:var(--text-primary);background-color:var(--glass-selected);border-color:var(--glass-edge);box-shadow:inset 0 1px 0 var(--bevel-top),0 2px 0 var(--pill-ledge);transform:translateY(-1px)}
.cat-btn:active{transform:translateY(1.5px);box-shadow:0 0.5px 0 var(--pill-ledge)}
.cat-btn[aria-pressed="true"]{color:var(--glider-text);font-weight:700}
.cat-btn[aria-pressed="true"]:hover{color:var(--glider-text);background-color:transparent;border-color:transparent;box-shadow:none;transform:none}
.cat-btn.is-glider-fallback[aria-pressed="true"]{background-color:var(--glider-surface);color:var(--glider-text);border-color:var(--glider-edge);box-shadow:var(--glider-shadow)}
.cat-btn[aria-pressed="true"] .hi-icon{color:var(--glider-text);animation:catIconSpring 460ms cubic-bezier(0.16,1,0.3,1)}
.cat-count{font-size:0.75rem;font-weight:700;font-variant-numeric:tabular-nums;color:var(--text-muted);opacity:0.88;transition:color 200ms ease,opacity 200ms ease}
.cat-count:empty{display:none}
.cat-btn:hover .cat-count{color:var(--text-primary);opacity:0.95}
.cat-btn[aria-pressed="true"] .cat-count{color:var(--glider-text);opacity:0.72;font-weight:700}
.stream-list.is-cat-switching{animation:catStreamMorph 500ms cubic-bezier(0.16,1,0.3,1) both}
.inbox-grid{flex:1 1 0%;min-height:0;margin-top:10px;display:grid;grid-template-columns:clamp(300px,34vw,390px) minmax(0,1fr);grid-template-rows:minmax(0,1fr);gap:var(--col-gap);align-items:stretch;overflow:hidden}
.stream-pane{background-color:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-base);box-shadow:0 2px 10px rgba(17,17,15,0.05);height:100%;min-height:0;overflow-x:hidden;overflow-y:auto;overscroll-behavior:contain;scrollbar-gutter:stable}
.reader-pane{background-color:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-base);box-shadow:0 2px 10px rgba(17,17,15,0.05);height:100%;min-height:0;overflow:hidden;display:flex;flex-direction:column;padding:20px 24px 16px}
.stream-list{list-style:none;margin:0;padding:0;overflow-x: clip}
.stream-row{position:relative;padding:13px 16px 13px 20px;border-bottom:1px solid var(--row-border);overflow-x: clip;cursor:pointer;transition:background-color 280ms ease}
.stream-list.is-cat-switching .stream-row{animation:rowFadeUp 460ms cubic-bezier(0.16,1,0.3,1) both;animation-delay:calc(var(--row-idx,0) * 32ms)}
.stream-row::before{content:"";position:absolute;left:6px;top:12px;bottom:12px;width:3px;border-radius:999px;background-color:var(--text-accent);opacity:0;transform:scaleY(0.4);transition:transform 340ms cubic-bezier(0.16,1,0.3,1),opacity 220ms ease}
.stream-row:hover{background-color:var(--row-hover)}
.stream-row:last-child{border-bottom:none}
.stream-row[aria-selected="true"]{background-color:var(--row-selected)}
.stream-row[aria-selected="true"]::before{opacity:1;transform:scaleY(1)}
.stream-row-unread .row-sender-name,.stream-row-unread .row-subject{font-weight:700}
.stream-row-unread .sender-avatar{color:var(--text-accent)}
.row-header{display:flex;align-items:flex-start;gap:10px}
.sender-avatar{width:34px;height:34px;border-radius:50%;flex-shrink:0;display:inline-flex;align-items:center;justify-content:center;background-color:var(--bg-subtle);border:1px solid var(--border-subtle);color:var(--text-primary);overflow:hidden;margin-top:1px}
.sender-avatar-img{width:100%;height:100%;object-fit:cover;display:block}
.sender-avatar-initial{font-size:0.875rem;font-weight:700;color:var(--text-accent);line-height:1}
.sender-info{flex:1 1 auto;min-width:0}
.sender-top-line{display:flex;align-items:baseline;justify-content:space-between;gap:8px}
.row-sender-name{font-size:0.9rem;font-weight:700;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;min-width:0}
.row-sender-email{font-size:0.75rem;line-height:1.25;color:var(--text-muted);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.row-time{flex-shrink:0;white-space:nowrap;font-variant-numeric:tabular-nums;font-size:0.75rem;color:var(--text-muted)}
.row-subject{margin-top:6px;font-size:0.90625rem;line-height:1.35;color:var(--text-primary);display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden}
.row-snippet{margin-top:3px;font-size:0.8125rem;line-height:1.42;color:var(--text-muted);display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden}
.stream-empty{padding:48px 24px;text-align:center;color:var(--text-secondary)}
.stream-empty-title{margin-top:10px;font-size:1rem;font-weight:700;color:var(--text-primary)}
.stream-empty-desc{margin-top:6px;margin-bottom:16px;font-size:0.875rem;color:var(--text-secondary)}
.reader-empty{margin:auto;padding:40px 24px;text-align:center;color:var(--text-muted)}
.reader-content-wrap{display:flex;flex-direction:column;flex:1 1 0%;min-height:0}
.reader-animate #readerSubject{animation:readerStageIn 580ms cubic-bezier(0.16,1,0.3,1) both}
.reader-animate .reader-meta-row{animation:readerStageIn 620ms cubic-bezier(0.16,1,0.3,1) 70ms both}
.reader-animate .reader-actions{animation:readerStageIn 640ms cubic-bezier(0.16,1,0.3,1) 130ms both}
.reader-animate .reader-html-frame,.reader-animate .reader-body{transform-origin:top center;animation:readerCanvasUnfold 760ms cubic-bezier(0.16,1,0.3,1) 170ms both}
.mobile-back-btn{display:none;margin-bottom:12px;align-self:flex-start}
.reader-meta-row{margin-top:10px;display:flex;flex-wrap:wrap;align-items:center;justify-content:space-between;gap:8px 16px;font-size:0.875rem;color:var(--text-secondary)}
.reader-sender-block{display:flex;align-items:center;gap:10px;min-width:0}
.reader-sender-stack{display:flex;flex-direction:column;min-width:0}
.reader-sender-strong{font-weight:700;font-size:0.9375rem;color:var(--text-primary);line-height:1.25}
.reader-sender-sub{font-size:0.78125rem;color:var(--text-muted);line-height:1.3}
.reader-actions{flex-shrink:0;display:flex;flex-wrap:wrap;gap:8px;margin-top:12px;padding-bottom:12px;border-bottom:1px solid var(--border-subtle)}
.phishing-box{flex-shrink:0;margin-top:10px;padding:10px 14px;background-color:var(--bg-alert);border:1px solid var(--border-danger);border-radius:var(--radius-base);color:var(--text-danger);font-size:0.875rem;display:flex;align-items:center;gap:10px}
.reader-html-frame{flex:1 1 0%;min-height:0;width:100%;border:none;margin-top:8px;background-color:var(--bg-surface)}
.reader-body{flex:1 1 0%;min-height:0;overflow-y:auto;overscroll-behavior:contain;margin-top:14px;white-space:pre-wrap;overflow-wrap:break-word;font-size:0.96875rem;line-height:1.68;color:var(--text-primary)}
.ai-dialog{margin:auto;width:min(480px,calc(100% - 32px));max-height:calc(100dvh - 32px);overflow-y:auto;background:var(--bg-surface);color:var(--text-primary);border:1px solid var(--border-subtle);border-radius:20px;padding:24px;box-shadow:var(--glass-shadow)}
.ai-dialog::backdrop{background:var(--bg-backdrop)}
.ai-check{display:flex;align-items:center;gap:8px;font-size:.875rem;overflow-wrap:anywhere}
.ai-check input{flex-shrink:0;accent-color:var(--text-primary)}
#aiAccounts{display:grid;gap:8px}
.ai-summary{border:1px solid var(--border-subtle);background:var(--bg-notice);border-radius:var(--radius-base);padding:12px;margin-top:12px;white-space:pre-wrap;overflow-wrap:anywhere;font-size:.875rem;max-height:180px;overflow-y:auto}
.ai-workbench{flex-shrink:0;margin-top:8px;padding:11px 16px;background-color:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-base);animation:panelEnter 220ms cubic-bezier(0.22,1,0.36,1)}
.ai-wb-head{display:flex;align-items:center;justify-content:space-between;gap:10px}
.ai-wb-title-row{display:flex;align-items:center;gap:8px;min-width:0;font-size:0.875rem;color:var(--text-primary)}
.ai-wb-status{overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.ai-wb-meta{display:flex;align-items:center;gap:8px;flex-shrink:0;font-size:0.8125rem;font-variant-numeric:tabular-nums;color:var(--text-secondary)}
.ai-wb-bar-track{margin-top:7px;height:6px;width:100%;background-color:var(--bg-subtle);border-radius:999px;overflow:hidden}
.ai-wb-bar-fill{height:100%;width:0%;background-color:var(--text-primary);border-radius:999px;transition:width 150ms ease}
.ai-wb-live{margin-top:6px;font-size:0.8125rem;color:var(--text-secondary);white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
.ai-wb-digest{margin-top:8px;padding-top:8px;border-top:1px solid var(--border-subtle);display:flex;flex-direction:column;gap:2px;max-height:148px;overflow-y:auto}
.ai-wb-digest-head{font-size:0.78125rem;color:var(--text-muted);margin-bottom:2px}
.ai-wb-digest-row{appearance:none;width:100%;text-align:left;background:transparent;border:none;border-bottom:1px solid var(--row-border);padding:6px 6px;border-radius:6px;font-family:inherit;font-size:0.8125rem;color:var(--text-primary);cursor:pointer;display:flex;align-items:baseline;justify-content:space-between;gap:10px}
.ai-wb-digest-row:last-child{border-bottom:none}
.ai-wb-digest-row:hover{background-color:var(--row-hover)}
.ai-wb-digest-main{min-width:0;flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.ai-wb-digest-time{flex-shrink:0;font-size:0.75rem;color:var(--text-muted);font-variant-numeric:tabular-nums}
.stream-row.is-ai-scanning{background-color:var(--row-selected);outline:1.5px solid var(--border-strong);outline-offset:-1.5px}
.toast-bar{position:fixed;right:20px;bottom:20px;z-index:900;background-color:var(--bg-inverse);color:var(--text-inverse);padding:10px 16px;border-radius:var(--radius-base);font-size:0.875rem;font-weight:700;display:inline-flex;align-items:center;gap:8px;transform:translateY(16px);opacity:0;pointer-events:none;transition:transform 200ms cubic-bezier(0.22,1,0.36,1),opacity 200ms ease}
.toast-bar.is-visible{transform:translateY(0);opacity:1}
.hidden,[hidden]{display:none !important}
@media (max-width:1120px){
.topbar{grid-template-columns:auto minmax(0,1fr) auto auto}
.mailbox-controls{justify-self:end}
.search-wrap{order:5;grid-column:1 / -1}
.cat-btn{padding:0 9px;gap:4px}
#btnAutoSortLabel{display:none}
.filter-bar-right #btnAutoSort{width:36px;padding:0;justify-content:center}
}
@media (max-width:880px){
.shell{padding:12px 14px 20px}
.inbox-grid{grid-template-columns:280px minmax(0,1fr);gap:12px}
.reader-pane{padding:18px 20px}
#btnAiRunLabel{display:none}
.filter-bar-right #btnAiRun{width:36px;padding:0;justify-content:center}
}
@media (max-width:720px){
.provider-grid{grid-template-columns:repeat(2,minmax(0,1fr))}
.onboarding-wrap{padding:20px 18px;margin-top:12px}
.inbox-grid{grid-template-columns:minmax(0,1fr)}
.mobile-back-btn{display:inline-flex}
.inbox-grid:not(.has-mobile-reader) .reader-pane,.inbox-grid.has-mobile-reader .stream-pane{display:none}
}
@media (max-width:640px){
.topbar{grid-template-columns:minmax(0,1fr) auto auto;gap:10px}
.language-controls{grid-column:2;grid-row:1}
.window-controls{grid-column:3;grid-row:1}
.search-wrap{order:0;grid-column:1 / -1;grid-row:2}
.mailbox-controls{grid-column:1 / -1;grid-row:3;justify-self:stretch}
.acct-switcher{flex:1;min-width:0}
.topbar .acct-trigger{width:100%;max-width:none;justify-content:flex-start}
.acct-chevron,.topbar .btn-primary{margin-left:auto}
.acct-menu{left:0;right:auto}
}
@supports not (backdrop-filter:blur(1px)){.glass-pill{background:var(--bg-surface);border-color:var(--border-subtle)} }
@media (prefers-reduced-transparency:reduce){.glass-pill{background:var(--bg-surface);-webkit-backdrop-filter:none;backdrop-filter:none} }
@media (prefers-contrast:more){.glass-pill{background:var(--bg-surface);border-color:var(--text-secondary)} .cat-count{color:var(--text-secondary)} }
@media (prefers-reduced-motion: reduce){*,*::before,*::after{transition-duration:0.01ms !important;animation-duration:0.01ms !important} }
"##;
pub const INDEX_HTML: &str = r##"<!doctype html>
<html lang="vi">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Vortex</title>
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
<button id="btnAiSettings" type="button" class="btn btn-ghost" aria-haspopup="dialog" aria-label="Trợ lý AI" title="Trợ lý AI">AI</button>
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
<dialog id="aiDialog" class="ai-dialog" aria-labelledby="aiTitle">
<div class="onboarding-head"><h2 id="aiTitle" data-i18n="ai_assistant">Trợ lý AI</h2><button id="btnAiClose" type="button" class="btn btn-ghost" data-i18n="ai_close">Đóng</button></div>
<p class="field-hint" data-i18n="ai_privacy">Tắt mặc định. Chỉ xử lý hộp thư bạn chọn; dùng dịch vụ AI bên ngoài sẽ gửi nội dung đã che thông tin nhạy cảm đến dịch vụ đó.</p>
<form id="aiForm" class="form-stack">
<label class="ai-check"><input id="aiEnabled" type="checkbox"><span data-i18n="ai_enable">Bật trợ lý AI</span></label>
<label class="ai-check"><input id="aiAutoHide" type="checkbox"><span data-i18n="ai_hide">Tự ẩn thư quảng cáo chắc chắn, có thể khôi phục</span></label>
<div><label for="aiProvider" class="field-label" data-i18n="ai_provider">Dịch vụ</label><select id="aiProvider" class="input-field"><option value="ollama">Ollama</option><option value="llamacpp">llama.cpp</option><option value="openai">OpenAI compatible</option><option value="anthropic">Anthropic</option></select></div>
<div><label for="aiEndpoint" class="field-label" data-i18n="ai_endpoint">Địa chỉ dịch vụ</label><input id="aiEndpoint" type="url" class="input-field" required></div>
<div><label for="aiModel" class="field-label" data-i18n="ai_model">Mô hình</label><input id="aiModel" class="input-field" required maxlength="200"></div>
<div id="aiLocalModels" class="hidden"></div>
<div id="aiKeyField"><label for="aiKey" class="field-label" data-i18n="ai_key">API key (để trống để giữ khóa đã lưu)</label><input id="aiKey" type="password" class="input-field" autocomplete="new-password"></div>
<div><p class="field-label" data-i18n="ai_mailboxes">Hộp thư cho phép AI xử lý</p><div id="aiAccounts"></div></div>
<p id="aiSettingsStatus" class="field-hint" role="status"></p>
<button id="btnAiSave" class="btn btn-primary" type="submit" data-i18n="ai_save">Lưu cài đặt</button>
</form>
</dialog>
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
<div class="filter-bar glass-pill">
<nav id="categoryNav" class="category-pills" aria-label="Bộ lọc thư">
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
<button type="button" class="cat-btn" data-cat="__hidden" aria-pressed="false"><svg class="hi-icon" aria-hidden="true"><use href="#hi-archive-box"></use></svg><span data-i18n="ai_hidden">AI đã ẩn</span><span id="cnt_hidden" class="cat-count"></span></button>
</nav>
<div class="filter-bar-right">
<span id="statusNotice" class="otp-meta" role="status"></span>
<button id="btnAiRun" type="button" class="btn btn-sm sort-btn" aria-pressed="false" aria-label="Khởi động AI" title="Khởi động AI">
<svg class="hi-icon" aria-hidden="true"><use href="#hi-shield-check"></use></svg>
<span id="btnAiRunLabel" data-i18n="ai_run_btn">Khởi động AI</span>
</button>
<button id="btnAutoSort" type="button" class="btn btn-sm sort-btn" aria-pressed="false" aria-label="Tự động sắp xếp" title="Tự động sắp xếp">
<svg class="hi-icon" aria-hidden="true"><use href="#hi-arrows-up-down"></use></svg>
<span id="btnAutoSortLabel" data-i18n="auto_sort">Tự động sắp xếp</span>
</button>
</div>
</div>
<section id="aiWorkbench" class="ai-workbench hidden" aria-live="polite">
<div class="ai-wb-head">
<div class="ai-wb-title-row">
<svg id="aiWbIcon" class="hi-icon" aria-hidden="true"><use href="#hi-arrow-path"></use></svg>
<span id="aiWbStatus" class="ai-wb-status"></span>
</div>
<div class="ai-wb-meta">
<span id="aiWbProgressText"></span>
<button id="btnAiWbClose" type="button" class="btn btn-sm btn-ghost" data-i18n="ai_close">Đóng</button>
</div>
</div>
<div class="ai-wb-bar-track"><div id="aiWbBarFill" class="ai-wb-bar-fill"></div></div>
<p id="aiWbLiveItem" class="ai-wb-live"></p>
<div id="aiWbDigest" class="ai-wb-digest hidden"></div>
</section>
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
<button id="btnAiSummary" type="button" class="btn hidden" data-i18n="ai_summarize">Tóm tắt bằng AI</button>
<button id="btnAiRestore" type="button" class="btn hidden" data-i18n="ai_restore">Khôi phục thư</button>
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
<div id="readerAiSummary" class="ai-summary hidden" role="status" aria-live="polite"></div>
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
sync_now: "Làm mới", connect_account: "Thêm hộp thư", copy_code: "Sao chép mã", copy_all_codes: "Sao chép tất cả",
open_verify_link: "Mở liên kết xác nhận", open_body_link: "Mở liên kết trong thư", mobile_back_list: "Quay lại danh sách",
setup_heading: "Thêm hộp thư", setup_note: "Kết nối một hoặc nhiều hộp thư để nhận mã xác nhận (OTP) tự động.",
mode_single: "Thêm 1 tài khoản", mode_bulk: "Thêm nhiều tài khoản", bulk_label: "Danh sách tài khoản (mỗi dòng 1 tài khoản)",
bulk_hint: "Định dạng: email|mật_khẩu_ứng_dụng hoặc email:mật_khẩu. Tự động nhận cấu hình máy chủ và đồng bộ cùng lúc.",
bulk_submit: "Kết nối tất cả", sync_this_acc: "Lấy mã TK này", all_accounts: "Tất cả hộp thư",
add_account_menu: "Thêm hộp thư mới", remove_all_accounts: "Gỡ tất cả", remove_account_tip: "Gỡ hộp thư này",
google_login_btn: "Đăng nhập nhanh bằng Google", google_opening_cancel: "Đang mở cửa sổ Google... (Bấm để hủy)",
manual_divider: "Hoặc chọn dịch vụ để nhập mật khẩu ứng dụng:",
back_to_inbox: "Quay lại hộp thư", preset_custom: "Email khác", email_label: "Địa chỉ Email", host_label: "Máy chủ",
port_label: "Cổng", secret_label: "Mật khẩu ứng dụng", secret_placeholder: "Nhập mật khẩu ứng dụng",
show_pw: "Hiện", hide_pw: "Ẩn", save_connect: "Kết nối hộp thư", connecting_status: "Đang kết nối...",
syncing_new_mailbox: "Đang đồng bộ hộp thư mới...", adding_accounts: "Đang thêm tài khoản...",
cat_all: "Tất cả", cat_otp: "Mã OTP", cat_billing: "Hóa đơn", cat_shipping: "Đơn hàng", cat_security: "Bảo mật", cat_work: "Công việc", cat_promo: "Khuyến mãi", auto_sort: "Tự động sắp xếp",
ai_assistant: "Trợ lý AI", ai_enable: "Bật trợ lý AI", ai_close: "Đóng", ai_save: "Lưu cài đặt",
ai_privacy: "Tắt mặc định. Chỉ xử lý hộp thư bạn chọn. Dịch vụ bên ngoài chỉ nhận nội dung đã ẩn thông tin nhạy cảm.",
ai_hide: "Tự ẩn thư quảng cáo (luôn khôi phục lại được)", ai_provider: "Dịch vụ", ai_endpoint: "Địa chỉ dịch vụ", ai_model: "Mô hình",
ai_key: "API key (để trống nếu giữ khóa cũ)", ai_mailboxes: "Hộp thư cho phép AI xử lý", ai_hidden: "AI đã ẩn", ai_restore: "Khôi phục thư",
ai_summarize: "Tóm tắt bằng AI", ai_pending: "AI đang tóm tắt...", ai_saved: "Đã lưu cài đặt AI", ai_paused: "AI tạm dừng do giới hạn thiết bị hoặc pin.",
ai_timeout: "Chưa có kết quả. Bấm tóm tắt để thử lại.", ai_key_saved: "Đã lưu khóa trong bộ nhớ bảo mật của hệ điều hành.",
ai_summary_label: "Tóm tắt", ai_run_btn: "Khởi động AI", ai_running_btn: "AI đang chạy...",
ai_step_scan: "Đang phân loại hộp thư và lọc thư rác...", ai_step_digest: "Đang tổng hợp các thư quan trọng hôm nay...",
ai_today_head: "Thư quan trọng hôm nay (bấm vào mục bất kỳ để mở):", ai_no_today: "Hôm nay chưa có thư quan trọng mới. Các thư ưu tiên gần nhất:",
ai_hid_prefix: "Đã ẩn thư quảng cáo: ", ai_labeled_prefix: "Đã gắn nhãn",
ai_restored_toast: "Đã đưa thư về hộp thư chính",
theme_dark: "Tối", theme_light: "Sáng", theme_toggle_tip: "Đổi giao diện Sáng / Tối",
sfx_on_tip: "Âm thanh: Đang bật (Bấm để tắt)", sfx_off_tip: "Âm thanh: Đang tắt (Bấm để bật)",
select_item_prompt: "Chọn một thư ở bên trái để đọc nội dung", act_read: "Đã đọc", act_archive: "Lưu trữ", act_pin: "Ghim", act_unsub: "Hủy đăng ký", act_delete: "Xóa",
toast_archive: "Đã lưu trữ thư", toast_read: "Đã đánh dấu đã đọc", toast_pin: "Đã ghim thư", toast_unpin: "Đã bỏ ghim thư", toast_delete: "Đã xóa thư",
sort_auto_toast: "Đã xếp mã OTP và thư chưa đọc lên đầu", sort_time_toast: "Đã xếp theo thời gian mới nhất",
no_subject: "(Không tiêu đề)", pinned_tag: "[Ghim] ",
empty_no_items_title: "Chưa có thư trong mục này", empty_no_items_desc: "Bấm Làm mới hoặc chuyển sang mục khác ở thanh trên.",
search_placeholder: "Tìm người gửi, tiêu đề hoặc mã OTP...",
guide_gmail: "Mẹo: Dùng Mật khẩu ứng dụng 16 ký tự trong mục Bảo mật của tài khoản Google.",
guide_outlook: "Mẹo: Nhập địa chỉ Outlook/Hotmail cùng mật khẩu ứng dụng Microsoft.",
guide_icloud: "Mẹo: Tạo mật khẩu ứng dụng riêng tại trang quản lý Apple ID (appleid.apple.com).",
guide_custom_imap: "Mẹo: Nhập email, mật khẩu và địa chỉ máy chủ IMAP của bạn.",
win_min: "Thu nhỏ", win_max: "Phóng to", win_close: "Đóng"
},
en: {
sync_now: "Refresh", connect_account: "Add mailbox", copy_code: "Copy code", copy_all_codes: "Copy all",
open_verify_link: "Open verification link", open_body_link: "Open link in email", mobile_back_list: "Back to list",
setup_heading: "Add mailbox", setup_note: "Connect one or more mailboxes to receive verification codes automatically.",
mode_single: "Single account", mode_bulk: "Multiple accounts", bulk_label: "Account list (one per line)",
bulk_hint: "Format: email|app_password or email:password. Server settings are detected automatically and synced together.",
bulk_submit: "Connect all", sync_this_acc: "Sync this account", all_accounts: "All mailboxes",
add_account_menu: "Add mailbox", remove_all_accounts: "Remove all", remove_account_tip: "Remove this mailbox",
google_login_btn: "Sign in with Google", google_opening_cancel: "Opening Google Sign-In... (Click to cancel)",
manual_divider: "Or choose a provider to sign in with an app password:",
back_to_inbox: "Back to inbox", preset_custom: "Other email", email_label: "Email address", host_label: "Server",
port_label: "Port", secret_label: "App password", secret_placeholder: "Enter app password",
show_pw: "Show", hide_pw: "Hide", save_connect: "Connect mailbox", connecting_status: "Connecting...",
syncing_new_mailbox: "Syncing new mailbox...", adding_accounts: "Adding accounts...",
cat_all: "All", cat_otp: "OTP codes", cat_billing: "Billing", cat_shipping: "Orders", cat_security: "Security", cat_work: "Work", cat_promo: "Promotions", auto_sort: "Auto-sort",
ai_assistant: "AI assistant", ai_enable: "Enable AI assistant", ai_close: "Close", ai_save: "Save settings",
ai_privacy: "Off by default. Only processes mailboxes you select. External services receive messages with sensitive details redacted.",
ai_hide: "Auto-hide promotional mail (always recoverable)", ai_provider: "Provider", ai_endpoint: "Service endpoint", ai_model: "Model",
ai_key: "API key (leave blank to keep saved key)", ai_mailboxes: "Mailboxes AI can process", ai_hidden: "AI hidden", ai_restore: "Restore message",
ai_summarize: "Summarize with AI", ai_pending: "Summarizing...", ai_saved: "AI settings saved", ai_paused: "AI paused by device or battery limits.",
ai_timeout: "No result yet. Click summarize to try again.", ai_key_saved: "Key stored in your system credential manager.",
ai_summary_label: "Summary", ai_run_btn: "Start AI", ai_running_btn: "AI running...",
ai_step_scan: "Classifying your inbox and filtering spam...", ai_step_digest: "Summarizing today's important messages...",
ai_today_head: "Today's important messages (click any item to open):", ai_no_today: "No urgent messages arrived today yet. Latest priority messages:",
ai_hid_prefix: "Hid promotion: ", ai_labeled_prefix: "Labeled",
ai_restored_toast: "Moved message back to inbox",
theme_dark: "Dark", theme_light: "Light", theme_toggle_tip: "Switch Light / Dark theme",
sfx_on_tip: "Sound: On (Click to mute)", sfx_off_tip: "Sound: Off (Click to unmute)",
select_item_prompt: "Select a message on the left to read", act_read: "Mark read", act_archive: "Archive", act_pin: "Pin", act_unsub: "Unsubscribe", act_delete: "Delete",
toast_archive: "Archived message", toast_read: "Marked as read", toast_pin: "Pinned message", toast_unpin: "Unpinned message", toast_delete: "Deleted message",
sort_auto_toast: "Sorted OTP codes and unread messages first", sort_time_toast: "Sorted by newest time",
no_subject: "(No subject)", pinned_tag: "[Pinned] ",
empty_no_items_title: "No messages in this tab", empty_no_items_desc: "Click Refresh or pick another filter tab above.",
search_placeholder: "Search by sender, subject, or OTP...",
guide_gmail: "Tip: Use a 16-character App Password from your Google Account Security settings.",
guide_outlook: "Tip: Enter your Outlook address and Microsoft App Password.",
guide_icloud: "Tip: Create an app-specific password at appleid.apple.com.",
guide_custom_imap: "Tip: Enter your email, password, and IMAP server details.",
win_min: "Minimize", win_max: "Maximize", win_close: "Close"
}
};
const CAT_LABELS = {
vi: { code_otp: "Mã OTP", billing: "Hóa đơn", shipping: "Đơn hàng", security_alert: "Bảo mật", work: "Công việc", social: "Mạng xã hội", promo_newsletter: "Khuyến mãi", other: "Thư khác" },
en: { code_otp: "OTP", billing: "Billing", shipping: "Order", security_alert: "Security", work: "Work", social: "Social", promo_newsletter: "Promo", other: "General" }
};
let lang = "vi", theme = "light", sfxEnabled = true;
try {
const savedTheme = localStorage.getItem("vortex_theme");
if (savedTheme === "dark" || savedTheme === "light") theme = savedTheme;
else if (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches) theme = "dark";
if (localStorage.getItem("vortex_sfx") === "0") sfxEnabled = false;
} catch (_) {}
let audioCtx = null, customSfxBuffer = null, customSfxLoading = false, sfxFallbackAudio = null, lastSfxAt = 0;
function ensureAudioCtx() {
if (!audioCtx) { const Ctx = window.AudioContext || window.webkitAudioContext; if (!Ctx) return null; audioCtx = new Ctx(); }
if (audioCtx.state === "suspended") audioCtx.resume().catch(() => {});
if (!customSfxBuffer && !customSfxLoading && audioCtx) {
customSfxLoading = true;
try { fetch("/click.m4a").then((r) => r.arrayBuffer()).then((b) => audioCtx.decodeAudioData(b)).then((d) => { customSfxBuffer = d; }).catch(() => {}); } catch (_) {}
}
return audioCtx;
}
function playSampleSlice(ctx, offsetSec, durSec, rate, gainVal, delaySec = 0) {
if (!customSfxBuffer) {
if (!sfxFallbackAudio) sfxFallbackAudio = new Audio("/click.m4a");
const clone = sfxFallbackAudio.cloneNode();
clone.volume = Math.min(1, gainVal); clone.playbackRate = rate; clone.play().catch(() => {}); return;
}
const t0 = ctx.currentTime + delaySec, src = ctx.createBufferSource();
src.buffer = customSfxBuffer; src.playbackRate.setValueAtTime(rate, t0);
const vca = ctx.createGain(), playDur = Math.min(durSec, Math.max(0.04, customSfxBuffer.duration - offsetSec));
vca.gain.setValueAtTime(gainVal, t0); vca.gain.setValueAtTime(gainVal, t0 + Math.max(0.01, playDur - 0.025));
vca.gain.exponentialRampToValueAtTime(0.0001, t0 + playDur);
src.connect(vca); vca.connect(ctx.destination); src.start(t0, offsetSec, playDur);
}
function playCreamySfx(kind = "click") {
if (!sfxEnabled) return;
const nowMs = performance.now();
if (nowMs - lastSfxAt < (kind === "pill" ? 45 : 24)) return;
lastSfxAt = nowMs;
const ctx = ensureAudioCtx(); if (!ctx) return;
const j = 0.985 + Math.random() * 0.03;
if (kind === "pill") playSampleSlice(ctx, 0, 0.34, 1.04 * j, 0.9);
else if (kind === "select") playSampleSlice(ctx, 0, 0.32, 0.96 * j, 0.82);
else if (kind === "copy") { playSampleSlice(ctx, 0, 0.28, j, 0.88, 0); playSampleSlice(ctx, 0, 0.34, 1.18 * j, 0.92, 0.055); }
else if (kind === "delete") playSampleSlice(ctx, 0, 0.34, 0.84 * j, 0.9);
else playSampleSlice(ctx, 0, 0.34, j, 0.9);
}
let items = [], cachedAllItems = [], rawAllItemsCache = [], recentCodes = [];
let hiddenItemKeys = new Set(), aiHiddenKeys = new Set(), aiCatMap = {}, aiLabeledDone = false, aiWorkflowBusy = false;
try {
const rawHidden = JSON.parse(localStorage.getItem("vortex_hidden_items") || "[]");
if (Array.isArray(rawHidden)) rawHidden.forEach((k) => hiddenItemKeys.add(String(k)));
const rawAiHidden = JSON.parse(localStorage.getItem("vortex_ai_hidden") || "[]");
if (Array.isArray(rawAiHidden)) rawAiHidden.forEach((k) => aiHiddenKeys.add(String(k)));
const rawAiCats = JSON.parse(localStorage.getItem("vortex_ai_cats") || "{}");
if (rawAiCats && typeof rawAiCats === "object") { aiCatMap = rawAiCats; aiLabeledDone = Object.keys(aiCatMap).length > 0; }
} catch (_) {}
function itemHideKeys(it) {
if (!it) return [];
const keys = [`id:${it.id}`];
if (it.account || it.external_id) keys.push(`ext:${it.account || ""}:${it.external_id || ""}`);
return keys;
}
function markItemHidden(it) {
itemHideKeys(it).forEach((k) => hiddenItemKeys.add(k));
try { localStorage.setItem("vortex_hidden_items", JSON.stringify(Array.from(hiddenItemKeys).slice(-500))); } catch (_) {}
}
function isAiHidden(it) { return Boolean(it) && itemHideKeys(it).some((k) => aiHiddenKeys.has(k)); }
function markAiHidden(it) {
itemHideKeys(it).forEach((k) => aiHiddenKeys.add(k));
try { localStorage.setItem("vortex_ai_hidden", JSON.stringify(Array.from(aiHiddenKeys).slice(-500))); } catch (_) {}
}
function unmarkAiHidden(it) {
itemHideKeys(it).forEach((k) => aiHiddenKeys.delete(k));
try { localStorage.setItem("vortex_ai_hidden", JSON.stringify(Array.from(aiHiddenKeys).slice(-500))); } catch (_) {}
}
function applyAiCategoryOverride(it) {
if (!it) return it;
const k = `id:${it.id}`, extK = (it.account || it.external_id) ? `ext:${it.account || ""}:${it.external_id || ""}` : "";
const ov = aiCatMap[k] || (extK && aiCatMap[extK]);
if (ov) it.category = ov;
return it;
}
function isItemVisible(it) {
if (!it || it.state === "trash" || it.state === "archived" || it.state === "spam" || isAiHidden(it)) return false;
return !itemHideKeys(it).some((k) => hiddenItemKeys.has(k));
}
let accounts = [], selectedId = null, activeCategory = "", activeAccount = "", activePreset = "gmail", addMode = "single";
let searchTimer = null, toastTimer = null, lastStreamKey = "", lastRenderedItemId = null, lastRenderedHtmlLen = -1;
const autoEnrichedAccounts = new Set();
let syncInFlight = false, autoSortEnabled = false, aiSettings = { enabled: false, accounts: [] }, aiSummaryTimer = null, aiSummaryId = null;
let prevCatIndex = 0, catMorphTimer = null, lastStreamMorphAt = 0, lastReaderAnimAt = 0;
const ANIM_COOLDOWN_MS = 380, $ = (id) => document.getElementById(id);
function t(key) { return (I18N[lang] && I18N[lang][key]) || key; }
function catLabel(cat) { return (CAT_LABELS[lang] && CAT_LABELS[lang][cat]) || cat; }
function showToast(msg) {
const bar = $("toastBar"), txt = $("toastText"); if (!bar || !txt) return;
txt.textContent = msg; bar.classList.add("is-visible");
if (toastTimer) clearTimeout(toastTimer);
toastTimer = setTimeout(() => bar.classList.remove("is-visible"), 2200);
}
function cleanSenderName(raw) {
if (!raw) return "Gmail";
const s = String(raw).trim(), ltIdx = s.indexOf("<");
return (ltIdx > 0 ? s.slice(0, ltIdx).replace(/^["']|["']$/g, "") : s.replace(/^<|>$/g, "")).trim();
}
function cleanSenderEmail(raw) {
if (!raw) return "";
const m = String(raw).match(/<([^>]+)>/); if (m) return m[1].trim();
const s = String(raw).trim(); return s.includes("@") ? s : "";
}
function stripHtmlSourceToText(raw) {
let s = String(raw || "");
if (/&lt;(?:!doctype|html|head|style|body|table|div)/i.test(s)) { const ta = document.createElement("textarea"); ta.innerHTML = s; s = ta.value; }
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
svg.setAttribute("class", cls); svg.setAttribute("aria-hidden", "true");
const use = document.createElementNS("http://www.w3.org/2000/svg", "use");
use.setAttribute("href", "#" + name); svg.appendChild(use); return svg;
}
let googlePollTimer = null;
function resetGoogleLoginBtn() {
if (googlePollTimer) { clearInterval(googlePollTimer); googlePollTimer = null; }
if ($("btnGoogleLogin")) $("btnGoogleLogin").disabled = false;
if ($("btnGoogleLoginText")) $("btnGoogleLoginText").textContent = t("google_login_btn");
}
function setScreenMode(mode) {
const isSetup = mode === "setup"; resetGoogleLoginBtn();
$("onboardingView").classList.toggle("hidden", !isSetup); $("inboxView").classList.toggle("hidden", isSetup);
$("topSearchWrap").classList.toggle("hidden", isSetup); $("btnCloseSetup").classList.toggle("hidden", accounts.length === 0);
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
activePreset = preset; $("accPreset").value = preset;
document.querySelectorAll(".provider-grid .preset-btn").forEach((btn) => btn.setAttribute("aria-pressed", btn.getAttribute("data-preset") === preset ? "true" : "false"));
const u = lang === "vi" ? "tenban" : "you"; let showServer = false;
if (preset === "gmail") { $("accHost").value = "imap.gmail.com"; $("accPort").value = "993"; $("accEmail").placeholder = `${u}@gmail.com`; }
else if (preset === "outlook") { $("accHost").value = "outlook.office365.com"; $("accPort").value = "993"; $("accEmail").placeholder = `${u}@outlook.com`; }
else if (preset === "icloud") { $("accHost").value = "imap.mail.me.com"; $("accPort").value = "993"; $("accEmail").placeholder = `${u}@icloud.com`; }
else { showServer = true; $("accEmail").placeholder = `${u}@domain.com`; }
$("customServerWrap").classList.toggle("hidden", !showServer); $("providerGuide").textContent = t("guide_" + preset);
}
function closeAccountMenu() {
const menu = $("acctMenu"), btn = $("btnAcctMenu");
if (menu) menu.classList.add("hidden"); if (btn) btn.setAttribute("aria-expanded", "false");
}
function updateSetupButtonLabel() {
const lbl = $("btnToggleSetupLabel"); if (lbl) lbl.textContent = t("connect_account");
const switcher = $("acctSwitcher"), activeLbl = $("acctActiveLabel");
if (!switcher || !activeLbl) return;
if (accounts.length === 0) { switcher.classList.add("hidden"); closeAccountMenu(); return; }
switcher.classList.remove("hidden");
if (activeAccount) { const found = accounts.find((a) => a.id === activeAccount); activeLbl.textContent = found ? found.email_or_handle : activeAccount; }
else if (accounts.length === 1) activeLbl.textContent = accounts[0].email_or_handle;
else activeLbl.textContent = `${t("all_accounts")} (${accounts.length})`;
}
function updateCategoryGlider() {
const glider = $("catGlider"), activeBtn = document.querySelector('.category-pills .cat-btn[aria-pressed="true"]');
if (!glider || !activeBtn) return;
const w = activeBtn.offsetWidth, h = activeBtn.offsetHeight;
if (w <= 0 || h <= 0) { activeBtn.classList.add("is-glider-fallback"); return; }
activeBtn.classList.remove("is-glider-fallback");
glider.style.width = `${w}px`; glider.style.height = `${h}px`;
glider.style.transform = `translate3d(${activeBtn.offsetLeft}px, ${activeBtn.offsetTop}px, 0)`; glider.style.opacity = "1";
}
function triggerStreamMorph(dir = 1) {
const ul = $("streamList"); if (!ul) return;
const now = performance.now(); if (now - lastStreamMorphAt < ANIM_COOLDOWN_MS) return;
lastStreamMorphAt = now; ul.style.setProperty("--cat-dir", String(dir));
ul.classList.remove("is-cat-switching"); void ul.offsetWidth; ul.classList.add("is-cat-switching");
if (catMorphTimer) clearTimeout(catMorphTimer);
catMorphTimer = setTimeout(() => ul.classList.remove("is-cat-switching"), 820);
}
function sortItemsList(list) {
const copy = list.slice();
if (!autoSortEnabled) { copy.sort((a, b) => Boolean(a.pinned) !== Boolean(b.pinned) ? (a.pinned ? -1 : 1) : (b.received_at || 0) - (a.received_at || 0)); return copy; }
const catRank = { code_otp: 6, security_alert: 5, billing: 4, shipping: 3, work: 2, social: 1, other: 1, promo_newsletter: 0 };
copy.sort((a, b) => {
if (Boolean(a.pinned) !== Boolean(b.pinned)) return a.pinned ? -1 : 1;
const aHasOtp = a.category === "code_otp" || (a.extracted_codes && a.extracted_codes.length > 0);
const bHasOtp = b.category === "code_otp" || (b.extracted_codes && b.extracted_codes.length > 0);
if (aHasOtp !== bHasOtp) return aHasOtp ? -1 : 1;
if ((a.state === "unread") !== (b.state === "unread")) return a.state === "unread" ? -1 : 1;
if ((a.priority_score || 0) !== (b.priority_score || 0)) return (b.priority_score || 0) - (a.priority_score || 0);
const aRank = catRank[a.category] !== undefined ? catRank[a.category] : 1, bRank = catRank[b.category] !== undefined ? catRank[b.category] : 1;
return aRank !== bRank ? bRank - aRank : (b.received_at || 0) - (a.received_at || 0);
});
return copy;
}
function applySfxButton() {
const iconUse = $("sfxIconUse"); if (iconUse) iconUse.setAttribute("href", sfxEnabled ? "#hi-speaker-wave" : "#hi-speaker-x-mark");
const btn = $("btnSfx");
if (btn) { const tip = sfxEnabled ? t("sfx_on_tip") : t("sfx_off_tip"); btn.setAttribute("aria-label", tip); btn.title = tip; btn.style.opacity = sfxEnabled ? "1" : "0.55"; }
}
function applyTheme(rerenderIframe = false) {
document.documentElement.setAttribute("data-theme", theme);
if (window.ipc) window.ipc.postMessage("win:theme:" + theme);
const iconUse = $("themeIconUse"); if (iconUse) iconUse.setAttribute("href", theme === "dark" ? "#hi-sun" : "#hi-moon");
const lbl = $("themeLabel"); if (lbl) lbl.textContent = theme === "dark" ? t("theme_light") : t("theme_dark");
const btn = $("btnTheme"); if (btn) { btn.setAttribute("aria-label", t("theme_toggle_tip")); btn.title = t("theme_toggle_tip"); }
applySfxButton();
if (rerenderIframe && selectedId) { lastRenderedItemId = null; selectItem(selectedId, false); }
requestAnimationFrame(updateCategoryGlider);
}
function applyTranslations() {
document.documentElement.lang = lang;
$("btnAiSettings").setAttribute("aria-label", t("ai_assistant")); $("btnAiSettings").title = t("ai_assistant");
$("btnLang").setAttribute("aria-label", lang === "vi" ? "Switch to English" : "Chuyển sang tiếng Việt");
$("btnSync").setAttribute("aria-label", t("sync_now")); $("btnSync").title = t("sync_now");
$("searchInput").setAttribute("aria-label", t("search_placeholder")); $("langLabel").textContent = lang === "vi" ? "EN" : "VI";
if ($("btnWinMin")) { $("btnWinMin").setAttribute("aria-label", t("win_min")); $("btnWinMin").title = t("win_min"); }
if ($("btnWinMax")) { $("btnWinMax").setAttribute("aria-label", t("win_max")); $("btnWinMax").title = t("win_max"); }
if ($("btnWinClose")) { $("btnWinClose").setAttribute("aria-label", t("win_close")); $("btnWinClose").title = t("win_close"); }
if ($("btnAiRun")) { $("btnAiRun").setAttribute("aria-label", t("ai_run_btn")); $("btnAiRun").title = t("ai_run_btn"); }
if ($("btnAutoSort")) { $("btnAutoSort").setAttribute("aria-label", t("auto_sort")); $("btnAutoSort").title = t("auto_sort"); }
applyTheme(false);
document.querySelectorAll("[data-i18n]").forEach((el) => { const k = el.getAttribute("data-i18n"); if (I18N[lang][k]) el.textContent = I18N[lang][k]; });
if (googlePollTimer && $("btnGoogleLoginText")) $("btnGoogleLoginText").textContent = t("google_opening_cancel");
if ($("accSecret")) $("accSecret").placeholder = t("secret_placeholder");
if ($("btnTogglePw") && $("accSecret")) $("btnTogglePw").textContent = $("accSecret").type === "password" ? t("show_pw") : t("hide_pw");
selectPreset(activePreset); updateSetupButtonLabel(); $("searchInput").placeholder = t("search_placeholder");
if ($("aiDialog") && $("aiDialog").open) updateLocalModels();
if (!aiWorkflowBusy && $("aiWorkbench") && !$("aiWorkbench").classList.contains("hidden")) renderAiDigestSummary();
lastStreamKey = ""; renderAccountSwitcher(cachedAllItems); renderStream(false);
if (selectedId) { lastRenderedItemId = null; selectItem(selectedId, false); }
requestAnimationFrame(updateCategoryGlider);
}
async function api(path, opts = {}) {
const res = await fetch(path, { headers: { "Content-Type": "application/json" }, ...opts });
if (!res.ok) { const txt = await res.text(); throw new Error(txt || res.statusText); }
return res.json();
}
function updateAiReader(item) {
const enabled = aiSettings.enabled && aiSettings.accounts.includes(item.account);
$("btnAiSummary").classList.toggle("hidden", !enabled); $("btnAiRestore").classList.toggle("hidden", activeCategory !== "__hidden");
if (!enabled || aiSummaryId !== item.id) {
clearTimeout(aiSummaryTimer); aiSummaryId = null;
const summary = enabled && item.summary ? `${t("ai_summary_label")}: ${item.summary}` : "";
$("readerAiSummary").textContent = summary; $("readerAiSummary").classList.toggle("hidden", !summary);
}
}
function updateAiKeyField() {
const external = ["openai", "anthropic"].includes($("aiProvider").value);
$("aiKey").disabled = !external; $("aiKeyField").classList.toggle("hidden", !external);
}
async function loadAiSettings() {
aiSettings = await api("/api/ai/settings");
const item = items.find((it) => it.id === selectedId); if (item) updateAiReader(item);
return aiSettings;
}
async function pollAiSummary(id, attempt = 0) {
if (selectedId !== id || aiSummaryId !== id) return;
const box = $("readerAiSummary");
try {
const result = await api(`/api/ai/result?item_id=${id}`);
if (selectedId !== id || aiSummaryId !== id) return;
if (result.status === "pending" && attempt < 60) aiSummaryTimer = setTimeout(() => pollAiSummary(id, attempt + 1), 1000);
else if (result.status === "done") box.textContent = result.summary ? `${t("ai_summary_label")}: ${result.summary}` : "";
else box.textContent = result.last_error || t("ai_timeout");
} catch (err) { if (selectedId === id && aiSummaryId === id) box.textContent = err.message; }
}
function updateCategoryCounts(allItems) {
const scoped = activeAccount ? allItems.filter((it) => it.account === activeAccount) : allItems;
const rawScoped = activeAccount ? rawAllItemsCache.filter((it) => it.account === activeAccount) : rawAllItemsCache;
const counts = { all: scoped.length, code_otp: 0, billing: 0, shipping: 0, security_alert: 0, work: 0, promo_newsletter: 0, hidden: rawScoped.filter((it) => isAiHidden(it) || it.state === "spam").length };
scoped.forEach((it) => { if (counts[it.category] !== undefined) counts[it.category] += 1; });
let changed = false;
Object.keys(counts).forEach((k) => {
const el = $("cnt_" + k);
if (el) { const next = counts[k] > 0 ? String(counts[k]) : ""; if (el.textContent !== next) { el.textContent = next; changed = true; } }
});
if (changed) requestAnimationFrame(updateCategoryGlider);
}
async function loadAll(forceStreamAnim = false) {
try {
const qs = [];
if (activeCategory === "__hidden") qs.push("hidden=1");
else if (activeCategory) qs.push(`category=${encodeURIComponent(activeCategory)}`);
if (activeAccount) qs.push(`account=${encodeURIComponent(activeAccount)}`);
const [status, accs, rawAllItems, rawFilteredList, rawCodes] = await Promise.all([
api("/api/status"), api("/api/accounts"), api("/api/items"), api("/api/items" + (qs.length ? `?${qs.join("&")}` : "")), api("/api/codes")
]);
lastKnownAiRevision = status.ai_revision;
rawAllItems.forEach(applyAiCategoryOverride); rawFilteredList.forEach(applyAiCategoryOverride);
rawAllItemsCache = rawAllItems;
const allItems = rawAllItems.filter(isItemVisible);
let filteredList;
if (activeCategory === "__hidden") {
const byId = new Map();
rawFilteredList.forEach((it) => byId.set(it.id, it));
rawAllItems.filter((it) => (!activeAccount || it.account === activeAccount) && isAiHidden(it)).forEach((it) => byId.set(it.id, it));
filteredList = Array.from(byId.values());
} else if (activeCategory) filteredList = allItems.filter((it) => (!activeAccount || it.account === activeAccount) && it.category === activeCategory);
else filteredList = allItems.filter((it) => !activeAccount || it.account === activeAccount);
const codes = rawCodes.filter((c) => !hiddenItemKeys.has(`id:${c.item_id}`));
accounts = accs;
if (activeAccount && !accounts.some((a) => a.id === activeAccount)) activeAccount = "";
cachedAllItems = allItems; items = sortItemsList(filteredList); recentCodes = codes;
updateSetupButtonLabel(); updateCategoryCounts(allItems);
if (accounts.length === 0) setScreenMode("setup");
else if (status.missing_secret_accounts && status.missing_secret_accounts.length > 0 && rawAllItems.length === 0) {
const missingEmail = status.missing_secret_accounts[0];
setScreenMode("setup"); setAddMode("single"); $("accEmail").value = missingEmail;
$("setupErrorText").textContent = lang === "vi" ? `Vui lòng nhập Mật khẩu ứng dụng cho ${missingEmail} để tải thư về.` : `Please enter the App Password for ${missingEmail} to sync your messages.`;
$("setupErrorBox").classList.remove("hidden");
} else if (!$("onboardingView").classList.contains("hidden") && selectedId) setScreenMode("inbox");
renderAccountSwitcher(allItems);
if (forceStreamAnim) triggerStreamMorph(1);
renderStream(forceStreamAnim);
if (items.length === 0) {
selectedId = null; lastRenderedItemId = null;
$("readerContentWrap").classList.add("hidden"); $("readerEmptyState").classList.remove("hidden");
} else if (forceStreamAnim || !selectedId || !items.some((x) => x.id === selectedId)) {
if (window.innerWidth > 720) selectItem(items[0].id, false);
} else if (selectedId) selectItem(selectedId, false);
const needAcc = !googlePollTimer && allItems.find((i) => i.account && !autoEnrichedAccounts.has(i.account) && (!i.body_html || (i.body_text || "").length < 280));
if (needAcc) { autoEnrichedAccounts.add(needAcc.account); setTimeout(() => autoEnrichedAccounts.delete(needAcc.account), 18000); triggerSync(needAcc.account, true); }
} catch (err) { $("statusNotice").textContent = String(err.message || err); }
}
let syncSuccessTimer = null;
async function triggerSync(accountId = null, silent = false) {
if (syncInFlight) return;
syncInFlight = true;
const btn = $("btnSync"), useEl = btn ? btn.querySelector("use") : null;
if (!silent && btn) {
if (syncSuccessTimer) { clearTimeout(syncSuccessTimer); syncSuccessTimer = null; }
btn.classList.remove("is-synced"); if (useEl) useEl.setAttribute("href", "#hi-arrow-path"); btn.classList.add("is-spinning");
}
try {
const res = await api("/api/sync", { method: "POST", body: JSON.stringify(accountId ? { account_id: accountId } : {}) });
await loadAll();
[2600, 5600, 9500].forEach((ms) => setTimeout(loadAll, ms));
if (!silent && btn && !res.error) {
btn.classList.remove("is-spinning"); if (useEl) useEl.setAttribute("href", "#hi-check-circle"); btn.classList.add("is-synced");
syncSuccessTimer = setTimeout(() => { btn.classList.remove("is-synced"); if (useEl) useEl.setAttribute("href", "#hi-arrow-path"); syncSuccessTimer = null; }, 1200);
}
} catch (_) {} finally { syncInFlight = false; if (btn) btn.classList.remove("is-spinning"); }
}
function renderAccountSwitcher(allItems = []) {
const menu = $("acctMenu");
if (!menu) return;
menu.innerHTML = "";
if (accounts.length === 0) return;
const list = document.createElement("div"); list.className = "acct-menu-list";
const nowSec = Math.floor(Date.now() / 1000);
if (accounts.length > 1) {
const allWrap = document.createElement("div"); allWrap.className = "acct-item-wrap";
allWrap.setAttribute("data-active", activeAccount === "" ? "true" : "false");
const allBtn = document.createElement("button"); allBtn.type = "button"; allBtn.className = "acct-item-btn";
const allMain = document.createElement("div"); allMain.className = "acct-item-main";
const allText = document.createElement("div"); allText.className = "acct-item-text";
const allTitle = document.createElement("span"); allTitle.className = "acct-item-email"; allTitle.textContent = `${t("all_accounts")} (${accounts.length})`;
const allSub = document.createElement("span"); allSub.className = "acct-item-sub"; allSub.textContent = lang === "vi" ? `${allItems.length} thư tổng cộng` : `${allItems.length} messages total`;
allText.append(allTitle, allSub); allMain.append(svgIcon(activeAccount === "" ? "hi-check-circle" : "hi-user-group"), allText);
allBtn.appendChild(allMain);
allBtn.onclick = () => {
closeAccountMenu();
if (activeAccount === "" && $("onboardingView").classList.contains("hidden")) return;
activeAccount = ""; setScreenMode("inbox"); loadAll(true);
};
allWrap.appendChild(allBtn); list.appendChild(allWrap);
}
accounts.forEach((a) => {
const isSelected = activeAccount === a.id || (accounts.length === 1 && activeAccount === "");
const wrap = document.createElement("div"); wrap.className = "acct-item-wrap"; wrap.setAttribute("data-active", isSelected ? "true" : "false");
const btn = document.createElement("button"); btn.type = "button"; btn.className = "acct-item-btn";
const main = document.createElement("div"); main.className = "acct-item-main";
const textWrap = document.createElement("div"); textWrap.className = "acct-item-text";
const emailSpan = document.createElement("span"); emailSpan.className = "acct-item-email"; emailSpan.textContent = a.email_or_handle;
const mailCnt = allItems.filter((it) => it.account === a.id).length, activeOtp = recentCodes.find((c) => c.expires_at > nowSec && c.account === a.id);
const subSpan = document.createElement("span"); subSpan.className = "acct-item-sub";
subSpan.textContent = activeOtp ? (lang === "vi" ? `Mã OTP: ${activeOtp.code} · ${mailCnt} thư` : `OTP: ${activeOtp.code} · ${mailCnt} msgs`) : (lang === "vi" ? `${mailCnt} thư` : `${mailCnt} messages`);
textWrap.append(emailSpan, subSpan); main.append(svgIcon(isSelected ? "hi-check-circle" : "hi-user-circle"), textWrap); btn.appendChild(main);
btn.onclick = () => {
const nextAcc = accounts.length === 1 ? "" : a.id; closeAccountMenu();
if (activeAccount === nextAcc && $("onboardingView").classList.contains("hidden")) return;
activeAccount = nextAcc; setScreenMode("inbox"); loadAll(true);
};
const delBtn = document.createElement("button"); delBtn.type = "button"; delBtn.className = "acct-item-del";
delBtn.title = t("remove_account_tip"); delBtn.setAttribute("aria-label", t("remove_account_tip")); delBtn.appendChild(svgIcon("hi-trash"));
delBtn.onclick = async (e) => {
e.stopPropagation(); if (activeAccount === a.id) activeAccount = "";
await api("/api/accounts/remove", { method: "POST", body: JSON.stringify({ id: a.id }) }); await loadAll();
};
wrap.append(btn, delBtn); list.appendChild(wrap);
});
menu.appendChild(list);
const foot = document.createElement("div"); foot.className = "acct-menu-foot";
const addBtn = document.createElement("button"); addBtn.type = "button"; addBtn.className = "btn btn-sm";
addBtn.append(svgIcon("hi-user-plus"), document.createTextNode(t("add_account_menu")));
addBtn.onclick = () => { closeAccountMenu(); setScreenMode("setup"); };
foot.appendChild(addBtn);
if (accounts.length > 1) {
const clearBtn = document.createElement("button"); clearBtn.type = "button"; clearBtn.className = "btn btn-sm btn-danger";
clearBtn.append(svgIcon("hi-trash"), document.createTextNode(t("remove_all_accounts")));
clearBtn.onclick = async () => { closeAccountMenu(); await api("/api/accounts/remove", { method: "POST", body: JSON.stringify({ all: true }) }); activeAccount = ""; await loadAll(); };
foot.appendChild(clearBtn);
}
menu.appendChild(foot);
}
function createSenderAvatar(rawSender) {
const avatar = document.createElement("span"); avatar.className = "sender-avatar";
const name = cleanSenderName(rawSender), email = cleanSenderEmail(rawSender), domain = (email.split("@")[1] || "").toLowerCase();
const initial = ((name || email || "V").trim().charAt(0) || "V").toUpperCase();
let primaryUrl = "", secondaryUrl = "";
if (domain === "github.com" || domain.endsWith(".github.com")) {
const cleanHandle = name.replace(/\[bot\]$/i, "").trim();
if (cleanHandle.toLowerCase() === "github" || email.startsWith("noreply@")) primaryUrl = "https://github.com/github.png?size=64";
else if (/^[a-zA-Z0-9-]+$/.test(cleanHandle)) { primaryUrl = `https://github.com/${encodeURIComponent(cleanHandle)}.png?size=64`; secondaryUrl = "https://github.com/github.png?size=64"; }
else primaryUrl = "https://github.com/github.png?size=64";
} else if (domain && !/^(gmail\.com|googlemail\.com|outlook\.com|hotmail\.com|icloud\.com|yahoo\.com|live\.com)$/.test(domain)) {
primaryUrl = `https://www.google.com/s2/favicons?domain=${encodeURIComponent(domain)}&sz=64`; secondaryUrl = `https://icons.duckduckgo.com/ip3/${encodeURIComponent(domain)}.ico`;
} else if (email) primaryUrl = `https://unavatar.io/${encodeURIComponent(email)}?fallback=false`;
const showInitial = () => {
avatar.innerHTML = ""; const initSpan = document.createElement("span");
initSpan.className = "sender-avatar-initial"; initSpan.textContent = initial; avatar.appendChild(initSpan);
};
if (!primaryUrl) { showInitial(); return avatar; }
const img = document.createElement("img"); img.className = "sender-avatar-img"; img.src = primaryUrl; img.alt = name; img.loading = "lazy";
img.onerror = () => { if (secondaryUrl && img.src !== secondaryUrl) img.src = secondaryUrl; else showInitial(); };
avatar.appendChild(img); return avatar;
}
function renderStream(force = false) {
const ul = $("streamList");
const nextKey = items.map((i) => `${i.id}:${i.category || ""}:${aiLabeledDone ? 1 : 0}:${i.state}:${i.pinned ? 1 : 0}:${i.summary || ""}:${(i.subject || "").length}:${(i.body_text || "").length}:${(i.body_html || "").length}`).join("|");
if (!force && nextKey === lastStreamKey && ul.children.length > 0) { updateStreamSelectionAttr(); return; }
lastStreamKey = nextKey; ul.innerHTML = "";
if (!items.length) {
const li = document.createElement("li"); li.className = "stream-empty";
const title = document.createElement("div"); title.className = "stream-empty-title"; title.textContent = t("empty_no_items_title");
const desc = document.createElement("div"); desc.className = "stream-empty-desc"; desc.textContent = t("empty_no_items_desc");
li.append(svgIcon("hi-inbox", "hi-icon-lg"), title, desc); ul.appendChild(li); return;
}
items.forEach((item, idx) => {
const li = document.createElement("li");
li.className = "stream-row" + (item.state === "unread" ? " stream-row-unread" : "");
li.style.setProperty("--row-idx", String(Math.min(idx, 10)));
li.setAttribute("data-id", String(item.id)); li.setAttribute("aria-selected", item.id === selectedId ? "true" : "false"); li.tabIndex = 0;
const header = document.createElement("div"); header.className = "row-header";
const info = document.createElement("div"); info.className = "sender-info";
const topLine = document.createElement("div"); topLine.className = "sender-top-line";
const senderNameSpan = document.createElement("span"); senderNameSpan.className = "row-sender-name";
senderNameSpan.textContent = (item.pinned ? t("pinned_tag") : "") + cleanSenderName(item.sender);
const timeEl = document.createElement("span"); timeEl.className = "row-time";
timeEl.textContent = new Date(item.received_at * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
topLine.append(senderNameSpan, timeEl);
const emailLine = document.createElement("div"); emailLine.className = "row-sender-email";
const sEmail = cleanSenderEmail(item.sender);
const baseAddr = (!activeAccount && item.account && sEmail && sEmail !== item.account) ? `${sEmail} · ${item.account}` : (sEmail || item.account || "");
const showTag = aiLabeledDone || (item.category && item.category !== "other");
emailLine.textContent = showTag ? `[${catLabel(item.category)}] · ${baseAddr}` : baseAddr;
info.append(topLine, emailLine); header.append(createSenderAvatar(item.sender), info);
const subj = document.createElement("div"); subj.className = "row-subject"; subj.textContent = item.subject || t("no_subject");
const snip = document.createElement("div"); snip.className = "row-snippet";
snip.textContent = cleanBodyText(item.subject, item.summary || item.body_text || "").slice(0, 140);
li.append(header, subj, snip);
li.onclick = () => { playCreamySfx("select"); selectItem(item.id, true); };
li.onkeydown = (e) => { if (e.key === "Enter") { playCreamySfx("select"); selectItem(item.id, true); } };
ul.appendChild(li);
});
}
function updateStreamSelectionAttr() {
document.querySelectorAll("#streamList .stream-row").forEach((li) => li.setAttribute("aria-selected", Number(li.getAttribute("data-id")) === selectedId ? "true" : "false"));
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
const altBgHex = isDark ? "#252522" : "#e5e5dd", altBgRgb = isDark ? { r: 37, g: 37, b: 34, a: 1 } : { r: 229, g: 229, b: 221, a: 1 };
const fgHex = isDark ? "#ecece6" : "#181816", fgMutedHex = isDark ? "#c8c8c0" : "#363632", borderHex = isDark ? "#66665e" : "#85857c";
const isCellActive = (c) => ((c.textContent || "").trim().length > 0) || !!c.querySelector("img,svg,[role='img']");
const tableCandidates = new Set();
for (const tbl of Array.from(root.querySelectorAll("table"))) {
const rows = Array.from(tbl.rows || []); if (!rows.length) continue;
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
p.removeAttribute("border"); p.style.setProperty("border", "0", "important"); p.style.setProperty("padding", "0", "important");
}
p = p.parentElement;
}
rows.forEach((r, rIdx) => {
r.classList.add("vortex-data-row");
let cells = Array.from(r.cells || []);
if (cells.length === 1) {
const it = cells[0].querySelector(":scope > table");
if (it && it.rows && it.rows.length === 1 && Array.from(it.rows[0].cells || []).filter(isCellActive).length >= 2) {
it.style.setProperty("width", "100%", "important"); it.style.setProperty("border-collapse", "collapse", "important");
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
const tag = el.tagName, rawStyle = el.getAttribute("style") || "";
let currentBg = parentBg;
const parsedBg = parseEmailColor((el.style && el.style.backgroundColor) || el.getAttribute("bgcolor") || "");
if (parsedBg && parsedBg.a > 0.05) {
const blendedBg = blendEmailColor(parsedBg, parentBg), bgLum = emailRelLum(blendedBg);
const bgChroma = Math.max(blendedBg.r, blendedBg.g, blendedBg.b) - Math.min(blendedBg.r, blendedBg.g, blendedBg.b);
if (isDark && bgLum > 0.32 && bgChroma <= 48) {
el.removeAttribute("bgcolor"); el.style.setProperty("background-color", bgLum > 0.86 ? "#1b1b19" : altBgHex, "important"); currentBg = bgLum > 0.86 ? baseBg : altBgRgb;
} else if (!isDark && bgLum < 0.16 && bgChroma <= 40 && /^(TABLE|TBODY|THEAD|TR|TD|TH|DIV|SECTION|ARTICLE|CENTER|BODY)$/.test(tag)) {
el.removeAttribute("bgcolor"); el.style.setProperty("background-color", "#f4f4f0", "important"); currentBg = baseBg;
} else currentBg = blendedBg;
}
if (rawStyle && /border/i.test(rawStyle)) {
const bgLum = emailRelLum(currentBg);
["", "Top", "Right", "Bottom", "Left"].forEach((side) => {
const val = el.style["border" + side + "Color"];
if (val) { const bc = parseEmailColor(val); if (!bc || emailContrast(emailRelLum(blendEmailColor(bc, currentBg)), bgLum) < 2.5) el.style.setProperty("border-" + (side ? side.toLowerCase() + "-" : "") + "color", borderHex, "important"); }
});
if (tag === "TR") { const pTbl = el.closest("table"); if (pTbl) pTbl.style.setProperty("border-collapse", "collapse", "important"); }
}
if (el.hasAttribute("bordercolor")) { el.removeAttribute("bordercolor"); el.style.setProperty("border-color", borderHex, "important"); }
const bgLum = emailRelLum(currentBg), bgChroma = Math.max(currentBg.r, currentBg.g, currentBg.b) - Math.min(currentBg.r, currentBg.g, currentBg.b);
const isLink = tag === "A" || insideLink, colorAttr = el.getAttribute("color");
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
const fgBlended = blendEmailColor(parsedFg, currentBg), fgChroma = Math.max(fgBlended.r, fgBlended.g, fgBlended.b) - Math.min(fgBlended.r, fgBlended.g, fgBlended.b);
if (emailContrast(emailRelLum(fgBlended), bgLum) < 4.8 || fgChroma < 25) el.style.setProperty("color", targetLinkHex, "important");
}
}
} else if (parsedFg) {
const fgBlended = blendEmailColor(parsedFg, currentBg);
if (emailContrast(emailRelLum(fgBlended), bgLum) < 4.8) {
const isMuted = parsedFg.a < 0.72 || (emailRelLum(parsedFg) > 0.16 && emailRelLum(parsedFg) < 0.58);
el.style.setProperty("color", isMuted ? fgMutedHex : fgHex, "important");
}
} else if ((isDark && bgLum >= 0.5) || (!isDark && bgLum < 0.5)) el.style.setProperty("color", bgLum < 0.5 ? "#ecece6" : "#181816", "important");
for (let i = 0; i < el.children.length; i++) walk(el.children[i], currentBg, isLink);
};
walk(root, baseBg, false);
}
function buildEmailIframeDoc(rawHtml, plainText, animate = true) {
const isDark = theme === "dark";
let bodyContent = (rawHtml || "").replace(/\[remote image blocked\]/gi, "").trim();
if (/&lt;(?:!doctype|html|head|style|body|table|div|p|span|a\b)/i.test(bodyContent)) { const ta = document.createElement("textarea"); ta.innerHTML = bodyContent; bodyContent = ta.value; }
if (!bodyContent && /<(?:!doctype|html|head|style|body|table|div|p|span|a\b)/i.test(String(plainText || ""))) bodyContent = String(plainText || "");
if (/<[a-z!][\s\S]*>/i.test(bodyContent)) {
try {
const doc = new DOMParser().parseFromString(bodyContent, "text/html");
doc.querySelectorAll("script,style,head,meta,title,link,iframe,object,embed,svg,math,form,base,template,[hidden],[style*='display:none'],[style*='visibility:hidden']").forEach((n) => n.remove());
if (doc.body) {
doc.body.querySelectorAll("*").forEach((el) => Array.from(el.attributes || []).forEach((attr) => {
const an = attr.name.toLowerCase(), av = String(attr.value || "").trim();
if (an.startsWith("on") || an === "srcdoc" || an === "formaction" || ((an === "href" || an === "src") && !/^(?:https?:\/\/|mailto:|#|cid:)/i.test(av))) el.removeAttribute(attr.name);
}));
normalizeEmailDom(doc.body, isDark);
if (doc.body.innerHTML.trim()) bodyContent = doc.body.innerHTML.trim();
}
} catch (_) {}
}
if (!bodyContent) {
const escaped = String(plainText || "").replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/(https?:\/\/[^\s"'<>)]+)/g, '<a href="$1" target="_blank" rel="noopener noreferrer">$1</a>').replace(/\n/g, "<br>");
bodyContent = `<div style="line-height:1.65;font-size:15px;">${escaped}</div>`;
}
const bgHex = isDark ? "#1b1b19" : "#f4f4f0", surfaceAltHex = isDark ? "#252522" : "#e5e5dd", fgHex = isDark ? "#ecece6" : "#181816";
const linkHex = isDark ? "#6cb6ff" : "#0b57d0", borderHex = isDark ? "#66665e" : "#85857c", borderStrongHex = isDark ? "#85857c" : "#5c5c55";
const sbThumb = isDark ? "#4e4e48" : "#9e9e95", sbHover = isDark ? "#7c7c74" : "#62625b", sbActive = isDark ? "#ecece6" : "#1a1a18";
const bodyAnimRule = animate ? "body{animation:sheetImmersion 680ms cubic-bezier(0.16,1,0.3,1) both}" : "";
return `<!doctype html><html><head><meta charset="utf-8"><base target="_blank"><style>
@font-face{font-family:"Vortex SF Pro";src:local("SF Pro Display Bold"),local("SFProDisplay-Bold");font-weight:700}
@keyframes sheetImmersion{0%{opacity:0;transform:translate3d(0,12px,0)} 100%{opacity:1;transform:translate3d(0,0,0)}}
*,*::before,*::after{box-sizing:border-box}
*::-webkit-scrollbar{width:8px;height:0 !important;background:transparent}
*::-webkit-scrollbar:horizontal,*::-webkit-scrollbar-button,*::-webkit-scrollbar-corner{display:none !important;width:0 !important;height:0 !important}
*::-webkit-scrollbar-track{background:transparent;margin-block:6px}
*::-webkit-scrollbar-thumb{background-color:${sbThumb};border-radius:999px;border:2px solid transparent;background-clip:padding-box;min-height:36px}
*::-webkit-scrollbar-thumb:hover{background-color:${sbHover}}
*::-webkit-scrollbar-thumb:active{background-color:${sbActive}}
html,body{margin:0;padding:12px 6px 28px 4px;max-width:100%;overflow-x:hidden !important;font-family:"Vortex SF Pro",-apple-system,system-ui,"Segoe UI",Roboto,sans-serif;font-weight:700;font-size:15px;line-height:1.6;color:${fgHex};background-color:${bgHex};overflow-wrap:break-word}
${bodyAnimRule}
.blocked-image{display:none !important}
img{max-width:100% !important;height:auto !important}
a{color:${linkHex};text-decoration:underline;text-underline-offset:2px;overflow-wrap:break-word}
table{border-collapse:collapse;border-spacing:0}
table,tbody,tr,td,th,div,section,article,center{max-width:100% !important}
table.vortex-data-table{width:100% !important;border-collapse:collapse !important;border-spacing:0 !important;border:1.5px solid ${borderStrongHex} !important;margin:8px 0 !important}
table.vortex-stacked-table+table.vortex-stacked-table{margin-top:-1.5px !important}
.vortex-data-row{border-bottom:1px solid ${borderHex} !important}
.vortex-data-cell{border:1px solid ${borderHex} !important;padding:9px 12px !important;vertical-align:middle !important}
.vortex-data-head-cell,th{background-color:${surfaceAltHex} !important;border:1px solid ${borderHex} !important;border-bottom:1.5px solid ${borderStrongHex} !important;padding:9px 12px !important}
hr{border:0 !important;border-top:1.5px solid ${borderHex} !important;margin:12px 0 !important;opacity:1 !important}
blockquote{border-left:3px solid ${borderStrongHex} !important;margin:8px 0 !important;padding-left:12px !important}
body *{font-family:"Vortex SF Pro",-apple-system,system-ui,"Segoe UI",Roboto,sans-serif !important;font-weight:700 !important}
pre,code{white-space:pre-wrap !important;overflow-x:hidden !important;word-break:break-word}
</style></head><body>${bodyContent}</body></html>`;
}
function selectItem(id, openMobile = true) {
const changed = selectedId !== id;
selectedId = id; updateStreamSelectionAttr();
const item = items.find((i) => i.id === id);
if (!item) return;
updateAiReader(item);
if (openMobile) $("inboxGrid").classList.add("has-mobile-reader");
$("readerEmptyState").classList.add("hidden");
const wrap = $("readerContentWrap"); wrap.classList.remove("hidden");
const htmlLen = (item.body_html || "").length + (item.body_text || "").length;
if (!changed && lastRenderedItemId === id && lastRenderedHtmlLen === htmlLen) return;
lastRenderedItemId = id; lastRenderedHtmlLen = htmlLen;
const now = performance.now(), canAnimateReader = changed && (now - lastReaderAnimAt >= ANIM_COOLDOWN_MS);
if (canAnimateReader) {
lastReaderAnimAt = now; wrap.classList.remove("reader-animate"); void wrap.offsetWidth; wrap.classList.add("reader-animate");
}
$("readerSubject").textContent = item.subject || t("no_subject");
const metaEl = $("readerMeta"); metaEl.innerHTML = "";
const avatar = createSenderAvatar(item.sender);
const stack = document.createElement("div"); stack.className = "reader-sender-stack";
const strongName = document.createElement("span"); strongName.className = "reader-sender-strong"; strongName.textContent = cleanSenderName(item.sender);
const subLine = document.createElement("span"); subLine.className = "reader-sender-sub";
const emailPart = cleanSenderEmail(item.sender), timeStr = new Date(item.received_at * 1000).toLocaleString(), accTag = item.account ? ` · ${item.account}` : "";
subLine.textContent = (emailPart ? `${emailPart} · ` : "") + `${catLabel(item.category)}${accTag} · ${timeStr}`;
stack.append(strongName, subLine); metaEl.append(avatar, stack);
const phish = $("readerPhishing");
if (item.phishing_warning) { $("readerPhishingText").textContent = item.phishing_warning; phish.classList.remove("hidden"); }
else phish.classList.add("hidden");
const unsubBtn = $("btnUnsubscribe");
const safeUnsubUrl = item.unsubscribe_link && /^(?:https?:\/\/|mailto:)/i.test(String(item.unsubscribe_link).trim()) ? String(item.unsubscribe_link).trim() : null;
if (safeUnsubUrl) { unsubBtn.classList.remove("hidden"); unsubBtn.onclick = () => window.open(safeUnsubUrl, "_blank", "noopener,noreferrer"); }
else unsubBtn.classList.add("hidden");
const bodyTextClean = cleanBodyText(item.subject, item.body_text || "");
const linkBtn = $("btnOpenBodyLink"), firstUrlMatch = bodyTextClean.match(/https?:\/\/[^\s"'<>)]+/);
const rawActionLink = (item.extracted_links && item.extracted_links[0] && item.extracted_links[0].url) || (firstUrlMatch && firstUrlMatch[0]);
const actionLink = rawActionLink && /^https?:\/\//i.test(String(rawActionLink).trim()) ? String(rawActionLink).trim() : null;
if (actionLink && !item.phishing_warning) { linkBtn.classList.remove("hidden"); linkBtn.onclick = () => window.open(actionLink, "_blank", "noopener,noreferrer"); }
else linkBtn.classList.add("hidden");
const ent = $("readerEntities");
ent.textContent = (item.extracted_entities && item.extracted_entities.length) ? item.extracted_entities.map((e) => `${e.label}: ${e.value}`).join(" | ") : "";
const frame = $("readerHtmlFrame");
frame.classList.remove("hidden");
$("readerBody").classList.add("hidden");
frame.srcdoc = buildEmailIframeDoc(item.body_html, bodyTextClean, canAnimateReader);
if (!googlePollTimer && (!item.body_html || !/<(table|div|p|br|a\s)/i.test(item.body_html)) && (item.body_text || "").length < 300) triggerSync(item.account || null, true);
}
async function runAction(itemId, actionObj) {
if (!itemId) return;
const targetItem = items.find((i) => i.id === itemId) || cachedAllItems.find((i) => i.id === itemId);
let effectiveAction = (actionObj.action === "pin" && targetItem && targetItem.pinned) ? "unpin" : actionObj.action;
if (effectiveAction === "delete" || effectiveAction === "archive") {
playCreamySfx("delete");
if (targetItem) markItemHidden(targetItem);
const prevIdx = items.findIndex((i) => i.id === itemId);
items = items.filter((i) => i.id !== itemId);
cachedAllItems = cachedAllItems.filter((i) => i.id !== itemId);
recentCodes = recentCodes.filter((c) => c.item_id !== itemId);
updateCategoryCounts(cachedAllItems);
renderAccountSwitcher(cachedAllItems);
if (items.length > 0) selectItem(items[Math.min(Math.max(0, prevIdx), items.length - 1)].id, false);
else { selectedId = null; lastRenderedItemId = null; $("readerContentWrap").classList.add("hidden"); $("readerEmptyState").classList.remove("hidden"); }
renderStream(true);
}
await api("/api/actions", { method: "POST", body: JSON.stringify({ item_id: itemId, ...actionObj, action: effectiveAction }) });
const labels = { archive: t("toast_archive"), mark_read: t("toast_read"), pin: t("toast_pin"), unpin: t("toast_unpin"), delete: t("toast_delete") };
if (labels[effectiveAction]) showToast(labels[effectiveAction]);
await loadAll();
}
$("btnMobileBack").onclick = () => $("inboxGrid").classList.remove("has-mobile-reader");
async function updateLocalModels() {
const panel = $("aiLocalModels");
panel.classList.toggle("hidden", $("aiProvider").value !== "ollama");
if (panel.classList.contains("hidden")) return;
try { (await import("/local-models.js")).open(api); } catch (err) { panel.textContent = err.message; }
}
$("btnAiSettings").onclick = async () => {
const btn = $("btnAiSettings"); btn.disabled = true;
try {
const cfg = await loadAiSettings();
$("aiEnabled").checked = cfg.enabled; $("aiAutoHide").checked = cfg.auto_hide;
$("aiProvider").value = cfg.provider; $("aiEndpoint").value = cfg.endpoint; $("aiModel").value = cfg.small_model; $("aiKey").value = "";
updateAiKeyField();
$("aiSettingsStatus").textContent = cfg.last_error || (cfg.enabled && !cfg.allowed_now ? t("ai_paused") : cfg.has_api_key ? t("ai_key_saved") : "");
const list = $("aiAccounts"); list.replaceChildren();
if (!accounts.length) list.textContent = t("add_account_menu");
accounts.forEach((acc) => {
const label = document.createElement("label"); label.className = "ai-check";
const check = document.createElement("input"); check.type = "checkbox"; check.value = acc.id; check.checked = cfg.accounts.includes(acc.id);
label.append(check, document.createTextNode(acc.email_or_handle)); list.appendChild(label);
});
$("aiDialog").showModal(); updateLocalModels();
} catch (err) { showToast(err.message); } finally { btn.disabled = false; }
};
$("btnAiClose").onclick = () => { $("aiKey").value = ""; $("aiDialog").close(); };
$("aiDialog").addEventListener("close", () => { $("aiKey").value = ""; });
$("aiProvider").onchange = () => {
const defaults = { ollama: ["http://127.0.0.1:11434", "qwen2.5:1.5b"], llamacpp: ["http://127.0.0.1:8080", "local"], openai: ["https://api.openai.com", ""], anthropic: ["https://api.anthropic.com", ""] };
const value = defaults[$("aiProvider").value];
if (value) { $("aiEndpoint").value = value[0]; $("aiModel").value = value[1]; }
updateAiKeyField(); updateLocalModels();
};
$("aiEndpoint").addEventListener("change", updateLocalModels);
$("aiForm").onsubmit = async (event) => {
event.preventDefault();
const btn = $("btnAiSave"); btn.disabled = true;
const payload = { enabled: $("aiEnabled").checked, auto_hide: $("aiAutoHide").checked, provider: $("aiProvider").value, endpoint: $("aiEndpoint").value.trim(), small_model: $("aiModel").value.trim(), accounts: Array.from($("aiAccounts").querySelectorAll("input:checked"), (el) => el.value) };
if (!$("aiKey").disabled && $("aiKey").value.trim()) payload.api_key = $("aiKey").value.trim();
try {
aiSettings = await api("/api/ai/settings", { method: "POST", body: JSON.stringify(payload) });
$("aiKey").value = ""; $("aiDialog").close(); showToast(t("ai_saved")); await loadAll();
} catch (err) { $("aiSettingsStatus").textContent = err.message; } finally { btn.disabled = false; }
};
$("btnAiSummary").onclick = async () => {
if (!selectedId) return;
const id = selectedId, btn = $("btnAiSummary"), box = $("readerAiSummary");
btn.disabled = true; clearTimeout(aiSummaryTimer); aiSummaryId = id;
box.classList.remove("hidden"); box.textContent = t("ai_pending");
try { await api("/api/ai/summarize", { method: "POST", body: JSON.stringify({ id }) }); await pollAiSummary(id); }
catch (err) { if (selectedId === id) box.textContent = err.message; } finally { btn.disabled = false; }
};
$("btnAiRestore").onclick = async () => {
const id = selectedId; if (!id) return;
const found = items.find((i) => i.id === id) || rawAllItemsCache.find((i) => i.id === id);
if (found) unmarkAiHidden(found);
try {
await api("/api/ai/restore", { method: "POST", body: JSON.stringify({ id }) }).catch(() => {});
showToast(t("ai_restored_toast"));
await loadAll();
} catch (err) { showToast(err.message); }
};
function classifyItemForAi(it) {
const hay = `${it.sender || ""} ${it.subject || ""} ${cleanBodyText(it.subject, it.body_text || "")}`.toLowerCase();
if ((it.extracted_codes && it.extracted_codes.length > 0) || /\b(otp|mã xác nhận|mã xác thực|verification code|security code|passcode|login code|mã đăng nhập)\b/i.test(hay)) return { cat: "code_otp", hide: false };
if (/\b(cảnh báo bảo mật|bảo mật|đăng nhập mới|thiết bị mới|security alert|new sign-in|sign-in attempt|password|mật khẩu|xác minh|khôi phục|2-step|two-factor)\b/i.test(hay)) return { cat: "security_alert", hide: false };
if (/\b(hóa đơn|thanh toán|phương thức thanh toán|cập nhật phương thức|biên lai|billing|invoice|receipt|payment|subscription|overdue|gia hạn|thẻ tín dụng)\b/i.test(hay)) return { cat: "billing", hide: false };
if (/\b(đơn hàng|vận chuyển|giao hàng|order|shipped|delivery|tracking|kiện hàng|đặt hàng)\b/i.test(hay)) return { cat: "shipping", hide: false };
if (/\b(enquiry|website enquiry|budget|project|proposal|báo giá|hợp đồng|khách hàng|công việc|meeting|lịch họp|deadline|reference:|tuyển dụng|interview)\b/i.test(hay)) return { cat: "work", hide: false };
if (it.state === "spam" || it.category === "promo_newsletter" || /\b(khuyến mãi|giảm giá|voucher|coupon|deal|sale|flash sale|quảng cáo|unsubscribe|hủy đăng ký|newsletter|bản tin|ưu đãi|trúng thưởng|quà tặng)\b/i.test(hay)) return { cat: "promo_newsletter", hide: true };
return { cat: it.category || "work", hide: false };
}
function summarizeSingleMail(it) {
if (it.summary && it.summary.trim().length > 12) return it.summary.trim();
const sender = cleanSenderName(it.sender), subj = (it.subject || "").trim(), body = cleanBodyText(subj, it.body_text || "");
const otp = it.extracted_codes && it.extracted_codes[0] ? it.extracted_codes[0].code : "";
const firstSentence = (body.split(/(?<=[.!?])\s+/)[0] || body).slice(0, 120).trim();
if (otp) return lang === "vi" ? `[${catLabel(it.category)}] ${sender}: Mã ${otp} (${subj || firstSentence})` : `[${catLabel(it.category)}] ${sender}: Code ${otp} (${subj || firstSentence})`;
const detail = firstSentence && firstSentence.toLowerCase() !== subj.toLowerCase() ? `${subj} · ${firstSentence}` : (subj || firstSentence);
return `[${catLabel(it.category)}] ${sender}: ${detail}`;
}
let lastAiPoolLen = 0, lastAiHiddenCnt = 0;
function renderAiDigestSummary(poolLen = lastAiPoolLen, hiddenSpamCount = lastAiHiddenCnt) {
lastAiPoolLen = poolLen; lastAiHiddenCnt = hiddenSpamCount;
const statusEl = $("aiWbStatus"), liveEl = $("aiWbLiveItem"), digestEl = $("aiWbDigest");
const visibleNow = rawAllItemsCache.filter(isItemVisible);
const scopedVisible = activeAccount ? visibleNow.filter((i) => i.account === activeAccount) : visibleNow;
const nowDate = new Date(), todayStr = nowDate.toDateString(), nowSec = Math.floor(nowDate.getTime() / 1000);
const todayList = scopedVisible.filter((i) => i.received_at && (new Date(i.received_at * 1000).toDateString() === todayStr || (nowSec - i.received_at) <= 86400));
const rankedToday = sortItemsList(todayList.length > 0 ? todayList : scopedVisible).slice(0, 4);
digestEl.replaceChildren();
const head = document.createElement("div"); head.className = "ai-wb-digest-head"; head.textContent = todayList.length > 0 ? t("ai_today_head") : t("ai_no_today");
digestEl.appendChild(head);
rankedToday.forEach((it) => {
const btnRow = document.createElement("button"); btnRow.type = "button"; btnRow.className = "ai-wb-digest-row";
const mainSpan = document.createElement("span"); mainSpan.className = "ai-wb-digest-main"; mainSpan.textContent = summarizeSingleMail(it);
const timeSpan = document.createElement("span"); timeSpan.className = "ai-wb-digest-time"; timeSpan.textContent = new Date(it.received_at * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
btnRow.append(mainSpan, timeSpan);
btnRow.onclick = () => { playCreamySfx("select"); selectItem(it.id, true); };
digestEl.appendChild(btnRow);
});
digestEl.classList.remove("hidden");
statusEl.textContent = lang === "vi"
? `Đã gắn nhãn ${poolLen} thư, ẩn ${hiddenSpamCount} thư rác và tóm tắt ${rankedToday.length} thư quan trọng hôm nay.`
: `Labeled ${poolLen} messages, hid ${hiddenSpamCount} spam/promos, and summarized ${rankedToday.length} key messages for today.`;
liveEl.textContent = rankedToday[0] ? summarizeSingleMail(rankedToday[0]) : "";
return rankedToday;
}
$("btnAiWbClose").onclick = () => { $("aiWorkbench").classList.add("hidden"); $("btnAiRun").setAttribute("aria-pressed", "false"); };
$("btnAiRun").onclick = async () => {
if (aiWorkflowBusy) return;
const wb = $("aiWorkbench"), statusEl = $("aiWbStatus"), progEl = $("aiWbProgressText"), barEl = $("aiWbBarFill"), liveEl = $("aiWbLiveItem"), digestEl = $("aiWbDigest"), btn = $("btnAiRun"), lbl = $("btnAiRunLabel");
const pool = (activeAccount ? rawAllItemsCache.filter((i) => i.account === activeAccount) : rawAllItemsCache.slice())
.filter((i) => i.state !== "trash" && i.state !== "archived" && !itemHideKeys(i).some((k) => hiddenItemKeys.has(k)));
if (!pool.length) { showToast(t("empty_no_items_title")); return; }
aiWorkflowBusy = true; btn.setAttribute("aria-pressed", "true"); lbl.textContent = t("ai_running_btn");
wb.classList.remove("hidden"); wb.classList.add("is-spinning"); digestEl.classList.add("hidden"); digestEl.replaceChildren();
barEl.style.width = "0%"; statusEl.textContent = t("ai_step_scan");
const stepMs = Math.max(70, Math.min(165, Math.floor(2200 / Math.max(1, pool.length))));
let hiddenSpamCount = 0;
try {
for (let idx = 0; idx < pool.length; idx++) {
const it = pool[idx], pct = Math.round(((idx + 1) / pool.length) * 100);
barEl.style.width = `${pct}%`; progEl.textContent = `${idx + 1} / ${pool.length} (${pct}%)`;
document.querySelectorAll("#streamList .stream-row.is-ai-scanning").forEach((el) => el.classList.remove("is-ai-scanning"));
const rowEl = document.querySelector(`#streamList .stream-row[data-id="${it.id}"]`);
if (rowEl) { rowEl.classList.add("is-ai-scanning"); rowEl.scrollIntoView({ block: "nearest", behavior: "smooth" }); }
const res = classifyItemForAi(it);
it.category = res.cat; itemHideKeys(it).forEach((k) => { aiCatMap[k] = res.cat; }); aiLabeledDone = true;
if (res.hide) {
hiddenSpamCount += 1; markAiHidden(it);
liveEl.textContent = `${t("ai_hid_prefix")}${cleanSenderName(it.sender)} · ${it.subject || ""}`;
} else {
unmarkAiHidden(it);
liveEl.textContent = `${t("ai_labeled_prefix")}[${catLabel(it.category)}]: ${cleanSenderName(it.sender)} · ${it.subject || ""}`;
if (rowEl) {
const addrEl = rowEl.querySelector(".row-sender-email"), sEmail = cleanSenderEmail(it.sender);
const baseAddr = (!activeAccount && it.account && sEmail && sEmail !== it.account) ? `${sEmail} · ${it.account}` : (sEmail || it.account || "");
if (addrEl) addrEl.textContent = `[${catLabel(it.category)}] · ${baseAddr}`;
}
}
await new Promise((r) => setTimeout(r, stepMs));
}
document.querySelectorAll("#streamList .stream-row.is-ai-scanning").forEach((el) => el.classList.remove("is-ai-scanning"));
try { localStorage.setItem("vortex_ai_cats", JSON.stringify(aiCatMap)); } catch (_) {}
statusEl.textContent = t("ai_step_digest"); autoSortEnabled = true; $("btnAutoSort").setAttribute("aria-pressed", "true");
const visibleNow = rawAllItemsCache.filter(isItemVisible); cachedAllItems = visibleNow;
const scopedVisible = activeAccount ? visibleNow.filter((i) => i.account === activeAccount) : visibleNow;
items = sortItemsList(activeCategory === "__hidden" ? items : (activeCategory ? scopedVisible.filter((i) => i.category === activeCategory) : scopedVisible));
updateCategoryCounts(visibleNow); renderStream(true);
wb.classList.remove("is-spinning");
const rankedToday = renderAiDigestSummary(pool.length, hiddenSpamCount);
if (rankedToday[0]) {
selectItem(rankedToday[0].id, false);
if (aiSettings.enabled && aiSettings.accounts.includes(rankedToday[0].account)) {
api("/api/ai/summarize", { method: "POST", body: JSON.stringify({ id: rankedToday[0].id }) }).then(() => pollAiSummary(rankedToday[0].id)).catch(() => {});
}
}
playCreamySfx("copy");
} finally {
aiWorkflowBusy = false; wb.classList.remove("is-spinning"); lbl.textContent = t("ai_run_btn");
}
};
$("btnModeSingle").onclick = () => setAddMode("single");
$("btnModeBulk").onclick = () => setAddMode("bulk");
document.querySelectorAll(".provider-grid .preset-btn").forEach((btn) => { btn.onclick = () => selectPreset(btn.getAttribute("data-preset") || "gmail"); });
$("btnTogglePw").onclick = () => {
const inp = $("accSecret"), isPw = inp.type === "password";
inp.type = isPw ? "text" : "password";
$("btnTogglePw").textContent = isPw ? t("hide_pw") : t("show_pw");
};
let lastKnownAccCount = -1, lastKnownItemsCount = -1, lastKnownAiRevision = null;
$("btnGoogleLogin").onclick = async () => {
if (googlePollTimer) { resetGoogleLoginBtn(); return; }
$("setupErrorBox").classList.add("hidden");
const email = $("accEmail").value.trim(), prevIds = new Set(accounts.map((a) => a.id));
const sigOf = (arr) => arr.map((i) => `${i.id}:${(i.body_html || "").length}:${(i.body_text || "").length}`).join("|");
$("btnGoogleLoginText").textContent = t("google_opening_cancel");
try { await api("/api/google-login", { method: "POST", body: JSON.stringify({ email }) }); }
catch (err) { resetGoogleLoginBtn(); $("setupErrorText").textContent = String(err.message || err); $("setupErrorBox").classList.remove("hidden"); return; }
let pollTicks = 0, switchedInbox = false, lastSeenSig = sigOf(rawAllItemsCache);
googlePollTimer = setInterval(async () => {
if (++pollTicks > 160) { resetGoogleLoginBtn(); return; }
try {
const [accs, rawItems] = await Promise.all([api("/api/accounts"), api("/api/items")]);
const added = accs.find((a) => !prevIds.has(a.id));
const targetAcc = (added && added.id) || (email && accs.some((a) => a.id === email) ? email : "");
const nextSig = sigOf(rawItems);
if (added || (nextSig !== lastSeenSig && rawItems.length > 0) || (targetAcc && pollTicks >= 5)) {
lastSeenSig = nextSig;
if (!switchedInbox) { switchedInbox = true; if (targetAcc) activeAccount = targetAcc; setScreenMode("inbox"); }
await loadAll();
const accItems = targetAcc ? rawItems.filter((i) => i.account === targetAcc) : rawItems;
if (accItems.some((i) => (i.body_html || "").length > 30) || pollTicks >= 45) {
resetGoogleLoginBtn(); [1800, 4500].forEach((ms) => setTimeout(loadAll, ms));
}
}
} catch (_) {}
}, 450);
};
$("accountForm").onsubmit = async (e) => {
e.preventDefault(); $("setupErrorBox").classList.add("hidden");
const submitBtn = $("btnSubmitAccount"), submitTxt = $("btnSubmitAccountText");
submitBtn.disabled = true; submitTxt.textContent = t("connecting_status");
const email = $("accEmail").value.trim(), host = $("accHost").value.trim(), port = Number($("accPort").value) || 993, secret = $("accSecret").value;
try {
await api("/api/accounts", { method: "POST", body: JSON.stringify({ id: email, source: "email", email_or_handle: email, host, port, use_tls: port === 993 || port === 443, secret, async_sync: true }) });
$("accEmail").value = ""; $("accSecret").value = ""; activeAccount = email; setScreenMode("inbox");
$("statusNotice").textContent = t("syncing_new_mailbox");
await loadAll();
} catch (err) {
const msg = String(err.message || err);
$("setupErrorText").textContent = lang === "vi" ? `Không thể đăng nhập: ${msg}. Hãy kiểm tra lại Mật khẩu ứng dụng 16 ký tự.` : `Login failed: ${msg}. Please check your App Password.`;
$("setupErrorBox").classList.remove("hidden");
} finally { submitBtn.disabled = false; submitTxt.textContent = t("save_connect"); }
};
$("bulkAccountForm").onsubmit = async (e) => {
e.preventDefault(); $("setupErrorBox").classList.add("hidden");
const rawText = $("bulkAccountsInput").value.trim(); if (!rawText) return;
const btn = $("btnSubmitBulk"), txt = $("btnSubmitBulkText");
btn.disabled = true; txt.textContent = t("adding_accounts");
try {
const res = await api("/api/accounts/batch", { method: "POST", body: JSON.stringify({ text: rawText, default_host: $("accHost").value.trim() || "imap.gmail.com", default_port: Number($("accPort").value) || 993, async_sync: true }) });
$("bulkAccountsInput").value = ""; activeAccount = ""; setScreenMode("inbox");
showToast(lang === "vi" ? `Đã thêm ${res.added || 0} tài khoản, đang đồng bộ ngầm...` : `Added ${res.added || 0} accounts, syncing in background...`);
await loadAll();
} catch (err) { $("setupErrorText").textContent = String(err.message || err); $("setupErrorBox").classList.remove("hidden"); }
finally { btn.disabled = false; txt.textContent = t("bulk_submit"); }
};
$("btnAcctMenu").onclick = (e) => {
e.stopPropagation(); const menu = $("acctMenu"); if (!menu) return;
const isHidden = menu.classList.contains("hidden");
menu.classList.toggle("hidden", !isHidden); $("btnAcctMenu").setAttribute("aria-expanded", isHidden ? "true" : "false");
};
document.addEventListener("click", (e) => { const s = $("acctSwitcher"); if (s && !s.contains(e.target)) closeAccountMenu(); });
document.addEventListener("keydown", (e) => { if (e.key === "Escape") closeAccountMenu(); });
$("btnToggleSetup").onclick = () => { closeAccountMenu(); setScreenMode((!$("onboardingView").classList.contains("hidden") && accounts.length > 0) ? "inbox" : "setup"); };
$("btnCloseSetup").onclick = () => setScreenMode("inbox");
$("btnSync").onclick = () => triggerSync(activeAccount || null, false);
const postWinIpc = (msg) => { try { if (window.ipc && typeof window.ipc.postMessage === "function") window.ipc.postMessage(msg); } catch (_) {} };
if ($("btnWinMin")) $("btnWinMin").onclick = () => postWinIpc("win:min");
if ($("btnWinMax")) $("btnWinMax").onclick = () => postWinIpc("win:max");
if ($("btnWinClose")) $("btnWinClose").onclick = () => postWinIpc("win:close");
const topbarEl = document.querySelector(".topbar");
if (topbarEl) {
topbarEl.addEventListener("mousedown", (e) => { if (e.button === 0 && !e.target.closest("button, input, select, textarea, a, #acctMenu")) postWinIpc("win:drag"); });
topbarEl.addEventListener("dblclick", (e) => { if (!e.target.closest("button, input, select, textarea, a, #acctMenu")) postWinIpc("win:max"); });
}
$("btnSfx").onclick = () => {
sfxEnabled = !sfxEnabled; try { localStorage.setItem("vortex_sfx", sfxEnabled ? "1" : "0"); } catch (_) {}
applySfxButton(); if (sfxEnabled) playCreamySfx("copy");
};
$("btnTheme").onclick = (e) => {
const btn = $("btnTheme");
if (btn) {
btn.classList.remove("is-flipping"); void btn.offsetWidth; btn.classList.add("is-flipping");
setTimeout(() => btn.classList.remove("is-flipping"), 380);
}
const nextTheme = theme === "dark" ? "light" : "dark";
const commitTheme = () => {
theme = nextTheme;
try { localStorage.setItem("vortex_theme", theme); } catch (_) {}
applyTheme(true);
};
if (document.startViewTransition && !window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
const rect = btn ? btn.getBoundingClientRect() : null;
const x = rect ? rect.left + rect.width / 2 : (e.clientX || window.innerWidth / 2);
const y = rect ? rect.top + rect.height / 2 : (e.clientY || 0);
const endRadius = Math.hypot(Math.max(x, window.innerWidth - x), Math.max(y, window.innerHeight - y));
const transition = document.startViewTransition(() => commitTheme());
transition.ready.then(() => {
document.documentElement.animate(
{ clipPath: [`circle(0px at ${x}px ${y}px)`, `circle(${endRadius}px at ${x}px ${y}px)`] },
{ duration: 420, easing: "cubic-bezier(0.22, 1, 0.36, 1)", pseudoElement: "::view-transition-new(root)" }
);
}).catch(() => {});
} else {
document.documentElement.classList.add("theme-transitioning");
commitTheme();
setTimeout(() => document.documentElement.classList.remove("theme-transitioning"), 280);
}
};
document.addEventListener("pointerdown", (e) => {
const btn = e.target.closest("button");
if (!btn || btn.disabled || btn.id === "btnSfx") return;
if (btn.classList.contains("cat-btn") || btn.id === "btnAutoSort") playCreamySfx("pill");
else if (btn.classList.contains("otp-copy") || btn.id === "btnCopyAllOtp") playCreamySfx("copy");
else if (btn.getAttribute("data-act") === "delete" || btn.getAttribute("data-act") === "archive" || btn.classList.contains("acct-item-del")) playCreamySfx("delete");
else playCreamySfx("click");
});
let langSwitchBusy = false;
$("btnLang").onclick = () => {
if (langSwitchBusy) return;
langSwitchBusy = true;
const btn = $("btnLang");
if (btn) {
btn.classList.remove("is-switching"); void btn.offsetWidth; btn.classList.add("is-switching");
setTimeout(() => btn.classList.remove("is-switching"), 420);
}
const sel = "[data-i18n], #themeLabel, #acctActiveLabel, .stream-row .row-cat, .stream-row .row-time, .detail-meta-pill";
const targets = Array.from(document.querySelectorAll(sel)).filter((el) => el.offsetParent !== null);
targets.forEach((el, idx) => {
el.style.setProperty("--i18n-delay", `${Math.min(idx * 10, 90)}ms`);
el.classList.remove("i18n-wave-in"); el.classList.add("i18n-wave-out");
});
setTimeout(() => {
lang = lang === "vi" ? "en" : "vi";
try { applyTranslations(); } catch (_) {}
const freshTargets = Array.from(document.querySelectorAll(sel));
freshTargets.forEach((el, idx) => {
el.style.setProperty("--i18n-delay", `${Math.min(idx * 12, 110)}ms`);
el.classList.remove("i18n-wave-out"); el.classList.add("i18n-wave-in");
});
setTimeout(() => {
freshTargets.forEach((el) => { el.classList.remove("i18n-wave-in"); el.style.removeProperty("--i18n-delay"); });
langSwitchBusy = false;
}, 380);
}, 135);
};
$("searchInput").oninput = () => {
clearTimeout(searchTimer);
searchTimer = setTimeout(async () => {
const q = $("searchInput").value.trim();
if (!q) { await loadAll(); return; }
const accParam = activeAccount ? `&account=${encodeURIComponent(activeAccount)}` : "";
const hiddenParam = activeCategory === "__hidden" ? "&hidden=1" : "";
const searched = await api(`/api/items?q=${encodeURIComponent(q)}${accParam}${hiddenParam}`);
items = sortItemsList(activeCategory === "__hidden" ? searched : searched.filter(isItemVisible));
renderStream();
}, 180);
};
const catNav = $("categoryNav");
if (catNav) catNav.addEventListener("wheel", (e) => { if (Math.abs(e.deltaY) > Math.abs(e.deltaX) && catNav.scrollWidth > catNav.clientWidth) { e.preventDefault(); catNav.scrollLeft += e.deltaY; } }, { passive: false });
const catButtons = Array.from(document.querySelectorAll(".category-pills .cat-btn"));
catButtons.forEach((btn, idx) => {
btn.onclick = () => {
const nextCat = btn.getAttribute("data-cat") || "";
if (nextCat === activeCategory) return;
prevCatIndex = idx; catButtons.forEach((b) => b.setAttribute("aria-pressed", "false"));
btn.setAttribute("aria-pressed", "true"); btn.scrollIntoView({ behavior: "smooth", block: "nearest", inline: "nearest" });
updateCategoryGlider(); activeCategory = nextCat; loadAll(true);
};
});
$("btnAutoSort").onclick = () => {
if (performance.now() - lastStreamMorphAt < 220) return;
autoSortEnabled = !autoSortEnabled; $("btnAutoSort").setAttribute("aria-pressed", autoSortEnabled ? "true" : "false");
const prevTopId = items.length > 0 ? items[0].id : null;
items = sortItemsList(items);
const nextTopId = items.length > 0 ? items[0].id : null;
if (prevTopId !== nextTopId) triggerStreamMorph(1);
renderStream(prevTopId !== nextTopId);
if (items.length > 0 && window.innerWidth > 720 && selectedId !== items[0].id) selectItem(items[0].id, false);
showToast(autoSortEnabled ? t("sort_auto_toast") : t("sort_time_toast"));
};
window.addEventListener("resize", () => requestAnimationFrame(() => {
updateCategoryGlider();
const activeBtn = document.querySelector('.category-pills .cat-btn[aria-pressed="true"]');
if (activeBtn) activeBtn.scrollIntoView({ block: "nearest", inline: "nearest" });
}));
document.querySelectorAll("[data-act]").forEach((btn) => { btn.onclick = async () => { if (selectedId) await runAction(selectedId, { action: btn.getAttribute("data-act") }); }; });
setInterval(async () => {
try {
const st = await api("/api/status");
const accChanged = lastKnownAccCount !== -1 && st.accounts_count !== lastKnownAccCount;
const itemsChanged = lastKnownItemsCount !== -1 && st.items_count !== lastKnownItemsCount;
const aiChanged = lastKnownAiRevision !== null && st.ai_revision !== lastKnownAiRevision;
lastKnownAccCount = st.accounts_count; lastKnownItemsCount = st.items_count; lastKnownAiRevision = st.ai_revision;
if (!$("onboardingView").classList.contains("hidden") && st.accounts_count > 0 && (!st.missing_secret_accounts || st.missing_secret_accounts.length === 0) && accounts.length === 0) {
setScreenMode("inbox"); await loadAll(); return;
}
if (accChanged || itemsChanged || aiChanged) { await loadAll(); return; }
recentCodes = await api("/api/codes");
} catch (_) {}
}, 900);
setInterval(() => { if (accounts.length > 0 && !document.hidden) triggerSync(null, true); }, 10000);
let knownUiMtime = 0;
setInterval(async () => {
try {
const r = await api("/api/ui-mtime");
if (r && r.mtime) { if (knownUiMtime && r.mtime !== knownUiMtime) { location.reload(); return; } knownUiMtime = r.mtime; }
} catch (_) {}
}, 900);
applyTheme(false); selectPreset("gmail"); loadAll(); loadAiSettings().catch(() => {});
})();
"##;
pub const OTP_OVERLAY_HTML: &str = include_str!("otp_overlay.html");
pub const CLICK_AUDIO: &[u8] = include_bytes!("click.m4a");
pub const LOCAL_MODELS_JS: &str = include_str!("local_models.js");
/// Assets loaded by the main window; the overlay and click audio load separately.
pub const fn bundle_size_bytes() -> u64 {
(INDEX_HTML.len() + STYLES_CSS.len() + APP_JS.len()) as u64
}
#[cfg(test)]
mod tests {
use super::*;
use vortex_core::WEB_UI_MAX_ASSET_BYTES;
#[test]
fn bundle_stays_within_150kb_budget_and_hallmark_floor() {
assert!(bundle_size_bytes() <= WEB_UI_MAX_ASSET_BYTES);
assert!(OTP_OVERLAY_HTML.len() as u64 <= WEB_UI_MAX_ASSET_BYTES);
assert!(LOCAL_MODELS_JS.len() as u64 <= WEB_UI_MAX_ASSET_BYTES);
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
