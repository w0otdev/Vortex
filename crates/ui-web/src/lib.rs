pub const STYLES_CSS: &str = r##"/* hallmark · v2.0.0 · shape: app screen · genre: modern-minimal · theme: paper-ink (custom) · signature: live-otp-banner */
/* critique: calm two-pane casual inbox with centered single-purpose onboarding card and Heroicons 20/solid glyphs */
/* waive: custom-app-workbench */

:root {
  --bg-page: #f5f4f0;
  --bg-surface: #ffffff;
  --bg-subtle: #ebe9e2;
  --bg-inverse: #181816;
  --bg-alert: #fdf2f0;
  --bg-notice: #eaf4ee;
  --bg-backdrop: rgba(24, 24, 22, 0.45);
  --text-primary: #181816;
  --text-secondary: #52504a;
  --text-muted: #7a7870;
  --text-inverse: #f5f4f0;
  --text-danger: #a12c2c;
  --text-accent: #165640;
  --border-subtle: #e0ddd3;
  --border-strong: #181816;
  --border-danger: #cf5252;
  --font-sans: system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
  --font-mono: ui-monospace, "Cascadia Mono", "SFMono-Regular", Consolas, monospace;
  --radius-base: 8px;
  --container-max: 1280px;
  --col-gap: 16px;
}

*, *::before, *::after {
  box-sizing: border-box;
}

html, body {
  margin: 0;
  padding: 0;
  height: 100vh;
  overflow-x: clip;
  overflow-y: hidden;
  background-color: var(--bg-page);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: 16px;
  line-height: 1.5;
}

h1, h2, h3 {
  margin: 0;
  font-weight: 600;
  color: var(--text-primary);
  overflow-wrap: break-word;
  min-width: 0;
  letter-spacing: 0;
}

p {
  margin: 0;
}

button, input, select, textarea {
  font-family: var(--font-sans);
  font-size: 0.9375rem;
  color: var(--text-primary);
}

:focus-visible {
  outline: 2px solid var(--text-accent);
  outline-offset: 2px;
}

.svg-sprite {
  position: absolute;
  width: 0;
  height: 0;
  overflow: hidden;
  pointer-events: none;
}

.hi-icon {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  fill: currentColor;
  display: inline-block;
}

.hi-icon-lg {
  width: 22px;
  height: 22px;
  flex-shrink: 0;
  fill: currentColor;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes rowFadeUp {
  0% {
    opacity: 0;
    transform: translate3d(-12px, 12px, 0) scale(0.984);
    filter: blur(3px);
  }
  65% {
    opacity: 1;
    filter: blur(0);
  }
  100% {
    opacity: 1;
    transform: translate3d(0, 0, 0) scale(1);
    filter: blur(0);
  }
}

@keyframes readerStageIn {
  0% {
    opacity: 0;
    transform: translate3d(0, 18px, 0) scale(0.986);
    filter: blur(5px);
  }
  60% {
    opacity: 1;
    filter: blur(0.6px);
  }
  100% {
    opacity: 1;
    transform: translate3d(0, 0, 0) scale(1);
    filter: blur(0);
  }
}

@keyframes readerCanvasUnfold {
  0% {
    opacity: 0;
    transform: perspective(900px) translate3d(0, 26px, 0) rotateX(2.2deg) scale(0.982);
    filter: blur(6px);
  }
  65% {
    opacity: 1;
    filter: blur(0.5px);
  }
  100% {
    opacity: 1;
    transform: perspective(900px) translate3d(0, 0, 0) rotateX(0deg) scale(1);
    filter: blur(0);
  }
}

@keyframes panelEnter {
  from {
    opacity: 0;
    transform: scale(0.978) translateY(14px);
    filter: blur(4px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
    filter: blur(0);
  }
}

.is-spinning .hi-icon {
  animation: spin 700ms linear infinite;
}

.shell {
  max-width: var(--container-max);
  margin-inline: auto;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 14px 20px 14px;
}

.topbar {
  flex-shrink: 0;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 12px 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle);
}

.brand-mark {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  color: var(--text-accent);
  white-space: nowrap;
  flex-shrink: 0;
}

.brand-title {
  font-size: 1.25rem;
}

.search-wrap {
  position: relative;
  flex: 1 1 260px;
  max-width: 460px;
  min-width: 200px;
}

.search-wrap .hi-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  pointer-events: none;
  transition: color 150ms ease;
}

.search-wrap:focus-within .hi-icon {
  color: var(--text-accent);
}

.input-field {
  width: 100%;
  min-height: 40px;
  padding: 8px 14px;
  background-color: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-base);
  transition: border-color 150ms ease, box-shadow 150ms ease;
}

.input-field:focus {
  border-color: var(--text-accent);
  box-shadow: 0 0 0 3px rgba(22, 86, 64, 0.12);
  outline: none;
}

.search-wrap .input-field {
  padding-left: 38px;
}

.top-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.btn {
  appearance: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  min-height: 38px;
  padding: 0 14px;
  background-color: var(--bg-surface);
  color: var(--text-primary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-base);
  white-space: nowrap;
  cursor: pointer;
  transition: transform 140ms cubic-bezier(0.22, 1, 0.36, 1), background-color 140ms ease, border-color 140ms ease, opacity 140ms ease;
}

.btn:hover {
  background-color: var(--bg-subtle);
  transform: translateY(-1px);
}

.btn:active {
  transform: translateY(0) scale(0.97);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background-color: var(--bg-inverse);
  color: var(--text-inverse);
  border-color: var(--border-strong);
  font-weight: 500;
}

.btn-primary:hover {
  opacity: 0.92;
  background-color: var(--bg-inverse);
}

.btn-large {
  width: 100%;
  min-height: 44px;
  font-size: 1rem;
}

.btn-google {
  width: 100%;
  min-height: 46px;
  margin-top: 18px;
  font-size: 1rem;
  font-weight: 600;
  background-color: var(--text-accent);
  color: var(--text-inverse);
  border-color: var(--text-accent);
}

.btn-google:hover {
  opacity: 0.92;
  background-color: var(--text-accent);
}

.divider-caption {
  margin-top: 18px;
  padding-top: 14px;
  border-top: 1px solid var(--border-subtle);
  font-size: 0.8125rem;
  color: var(--text-muted);
}

.btn-danger {
  background-color: var(--bg-alert);
  color: var(--text-danger);
  border-color: var(--border-danger);
}

.btn-ghost {
  background-color: transparent;
  border-color: transparent;
  color: var(--text-secondary);
}

.btn-ghost:hover {
  background-color: var(--bg-subtle);
  color: var(--text-primary);
}

/* Instant Multi-Account OTP Banner */
.otp-banner {
  flex-shrink: 0;
  margin-top: 12px;
  padding: 10px 16px;
  background-color: var(--bg-notice);
  border: 1px solid var(--text-accent);
  border-radius: var(--radius-base);
  max-height: 220px;
  overflow-y: auto;
}

.otp-banner-head {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 8px 12px;
  padding-bottom: 8px;
  margin-bottom: 4px;
  border-bottom: 1px solid var(--border-subtle);
  font-size: 0.8125rem;
  font-weight: 600;
  color: var(--text-accent);
}

.otp-list {
  display: flex;
  flex-direction: column;
}

.otp-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 10px 16px;
  padding: 8px 0;
  border-bottom: 1px solid var(--border-subtle);
}

.otp-row:last-child {
  border-bottom: none;
  padding-bottom: 2px;
}

.otp-banner-left {
  display: flex;
  align-items: center;
  gap: 14px;
  min-width: 0;
}

.otp-code-value {
  font-family: var(--font-mono);
  font-size: 1.625rem;
  font-weight: 700;
  color: var(--text-accent);
  font-variant-numeric: tabular-nums;
  letter-spacing: 0.02em;
}

.otp-account-name {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-primary);
}

.otp-meta {
  font-size: 0.8125rem;
  color: var(--text-secondary);
}

/* Onboarding / Account Manager Centered View */
.onboarding-wrap {
  width: 100%;
  max-width: 640px;
  max-height: calc(100vh - 96px);
  overflow-y: auto;
  margin: 16px auto 0;
  background-color: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-base);
  padding: 24px 28px;
  animation: panelEnter 240ms cubic-bezier(0.22, 1, 0.36, 1);
}

.onboarding-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.onboarding-title {
  font-size: 1.375rem;
}

.onboarding-sub {
  margin-top: 6px;
  font-size: 0.9375rem;
  color: var(--text-secondary);
}

.mode-switch {
  margin-top: 18px;
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 8px;
}

.provider-grid {
  margin-top: 16px;
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
}

.preset-btn {
  appearance: none;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  min-height: 40px;
  padding: 0 12px;
  background-color: var(--bg-page);
  color: var(--text-primary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-base);
  font-size: 0.9375rem;
  cursor: pointer;
  transition: transform 140ms cubic-bezier(0.22, 1, 0.36, 1), background-color 140ms ease, color 140ms ease, border-color 140ms ease;
}

.preset-btn:hover {
  background-color: var(--bg-subtle);
  transform: translateY(-1px);
}

.preset-btn:active {
  transform: scale(0.97);
}

.preset-btn[aria-pressed="true"] {
  background-color: var(--bg-inverse);
  color: var(--text-inverse);
  border-color: var(--border-strong);
  font-weight: 600;
}

.form-stack {
  margin-top: 16px;
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.bulk-textarea {
  min-height: 120px;
  font-family: var(--font-mono);
  font-size: 0.875rem;
  line-height: 1.5;
  resize: vertical;
}

.field-label {
  display: block;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 6px;
}

.field-hint {
  margin-top: 6px;
  font-size: 0.8125rem;
  color: var(--text-secondary);
}

.secret-input-group {
  display: flex;
  gap: 8px;
}

.server-grid {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 12px;
}

.acct-switcher {
  position: relative;
  display: inline-flex;
}

.acct-trigger {
  max-width: 240px;
}

.acct-active-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.acct-menu {
  position: absolute;
  top: calc(100% + 6px);
  right: 0;
  width: 320px;
  max-height: 400px;
  overflow-y: auto;
  background-color: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-base);
  padding: 6px;
  z-index: 850;
  box-shadow: 0 12px 32px rgba(24, 24, 22, 0.14);
  animation: panelEnter 180ms cubic-bezier(0.16, 1, 0.3, 1);
}

.acct-menu-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.acct-item-wrap {
  display: flex;
  align-items: center;
  gap: 4px;
  border-radius: 6px;
  padding-right: 4px;
  transition: background-color 140ms ease;
}

.acct-item-wrap:hover {
  background-color: var(--bg-subtle);
}

.acct-item-wrap[data-active="true"] {
  background-color: var(--bg-subtle);
}

.acct-item-btn {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  padding: 8px 10px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--text-primary);
  font-family: inherit;
  font-size: 0.875rem;
  text-align: left;
  cursor: pointer;
}

.acct-item-wrap[data-active="true"] .acct-item-btn {
  font-weight: 600;
}

.acct-item-main {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.acct-item-text {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.acct-item-email {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  line-height: 1.3;
}

.acct-item-sub {
  font-size: 0.75rem;
  font-weight: 400;
  color: var(--text-muted);
  line-height: 1.3;
}

.acct-item-del {
  flex-shrink: 0;
  width: 28px;
  height: 28px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--text-muted);
  cursor: pointer;
  transition: color 140ms ease, background-color 140ms ease;
}

.acct-item-del:hover {
  color: var(--status-danger);
  background-color: var(--status-danger-bg);
}

.acct-menu-foot {
  margin-top: 6px;
  padding-top: 6px;
  border-top: 1px solid var(--border-subtle);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 6px;
}

.account-row-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 6px;
}

.btn-sm {
  min-height: 32px;
  padding: 0 10px;
  font-size: 0.8125rem;
}

/* Casual 2-Column Inbox View */
#inboxView {
  flex: 1 1 0%;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

@keyframes catIconSpring {
  0% {
    transform: scale(0.78) rotate(-8deg);
  }
  55% {
    transform: scale(1.14) rotate(3deg);
  }
  100% {
    transform: scale(1) rotate(0deg);
  }
}

@keyframes catStreamMorph {
  0% {
    opacity: 0;
    transform: translate3d(calc(var(--cat-dir, 1) * 18px), 8px, 0) scale(0.985);
    filter: blur(4px);
  }
  65% {
    opacity: 1;
    filter: blur(0);
  }
  100% {
    opacity: 1;
    transform: translate3d(0, 0, 0) scale(1);
    filter: blur(0);
  }
}

.filter-bar {
  flex-shrink: 0;
  margin-top: 10px;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.filter-bar-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.sort-btn[aria-pressed="true"] {
  background-color: var(--accent-soft);
  color: var(--text-accent);
  border-color: var(--accent-primary);
  font-weight: 600;
}

.sort-btn[aria-pressed="true"] .hi-icon {
  color: var(--text-accent);
  animation: catIconSpring 460ms cubic-bezier(0.16, 1, 0.3, 1);
}

.category-pills {
  position: relative;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 4px;
  padding: 3px;
  background-color: var(--bg-subtle);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-base);
}

.cat-glider {
  position: absolute;
  top: 0;
  left: 0;
  width: 0;
  height: 0;
  background-color: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(24, 24, 22, 0.06);
  pointer-events: none;
  z-index: 0;
  opacity: 0;
  transition: transform 420ms cubic-bezier(0.16, 1, 0.3, 1), width 420ms cubic-bezier(0.16, 1, 0.3, 1), height 420ms cubic-bezier(0.16, 1, 0.3, 1), opacity 200ms ease;
}

.cat-btn {
  position: relative;
  z-index: 1;
  appearance: none;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  min-height: 32px;
  padding: 0 12px;
  background-color: transparent;
  color: var(--text-secondary);
  border: 1px solid transparent;
  border-radius: 6px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: transform 180ms cubic-bezier(0.16, 1, 0.3, 1), color 200ms ease, background-color 180ms ease;
}

.cat-btn:hover {
  color: var(--text-primary);
}

.cat-btn:active {
  transform: scale(0.95);
}

.cat-btn[aria-pressed="true"] {
  color: var(--text-primary);
  font-weight: 600;
}

.cat-btn.is-glider-fallback[aria-pressed="true"] {
  background-color: var(--bg-surface);
  border-color: var(--border-subtle);
}

.cat-btn[aria-pressed="true"] .hi-icon {
  color: var(--text-accent);
  animation: catIconSpring 460ms cubic-bezier(0.16, 1, 0.3, 1);
}

.cat-count {
  font-size: 0.75rem;
  font-variant-numeric: tabular-nums;
  color: var(--text-muted);
  transition: color 200ms ease, transform 240ms cubic-bezier(0.16, 1, 0.3, 1);
}

.cat-btn[aria-pressed="true"] .cat-count {
  color: var(--text-accent);
  font-weight: 600;
}

.stream-list.is-cat-switching {
  animation: catStreamMorph 500ms cubic-bezier(0.16, 1, 0.3, 1) both;
}

.inbox-grid {
  flex: 1 1 0%;
  min-height: 0;
  margin-top: 10px;
  display: grid;
  grid-template-columns: clamp(300px, 34vw, 390px) minmax(0, 1fr);
  grid-template-rows: minmax(0, 1fr);
  gap: var(--col-gap);
  align-items: stretch;
  overflow: hidden;
}

.stream-pane {
  background-color: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-base);
  height: 100%;
  min-height: 0;
  overflow-y: auto;
  overscroll-behavior: contain;
}

.reader-pane {
  background-color: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-base);
  height: 100%;
  min-height: 0;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  padding: 20px 24px 16px;
}

.stream-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.stream-row {
  position: relative;
  padding: 13px 16px 13px 18px;
  border-bottom: 1px solid var(--border-subtle);
  cursor: pointer;
  transition: background-color 280ms ease, transform 380ms cubic-bezier(0.16, 1, 0.3, 1);
}

.stream-list.is-cat-switching .stream-row {
  animation: rowFadeUp 460ms cubic-bezier(0.16, 1, 0.3, 1) both;
  animation-delay: calc(var(--row-idx, 0) * 32ms);
}

.stream-row::before {
  content: "";
  position: absolute;
  left: 0;
  top: 6px;
  bottom: 6px;
  width: 3px;
  background-color: var(--text-accent);
  transform: scaleY(0);
  transition: transform 420ms cubic-bezier(0.16, 1, 0.3, 1);
}

.stream-row:hover {
  background-color: var(--bg-page);
  transform: translateX(2px);
}

.stream-row:last-child {
  border-bottom: none;
}

.stream-row[aria-selected="true"] {
  background-color: var(--bg-subtle);
  transform: translateX(4px);
}

.stream-row[aria-selected="true"]::before {
  transform: scaleY(1);
}

.stream-row-unread .row-sender-name,
.stream-row-unread .row-subject {
  font-weight: 700;
}

.stream-row-unread .sender-avatar {
  color: var(--text-accent);
}

.row-header {
  display: flex;
  align-items: flex-start;
  gap: 10px;
}

.sender-avatar {
  width: 34px;
  height: 34px;
  border-radius: 50%;
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-subtle);
  border: 1px solid var(--border-subtle);
  color: var(--text-primary);
  overflow: hidden;
  margin-top: 1px;
}

.sender-avatar-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.sender-avatar-initial {
  font-size: 0.875rem;
  font-weight: 700;
  color: var(--text-accent);
  line-height: 1;
}

.sender-info {
  flex: 1 1 auto;
  min-width: 0;
}

.sender-top-line {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 8px;
}

.row-sender-name {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.row-sender-email {
  font-size: 0.75rem;
  line-height: 1.25;
  color: var(--text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row-time {
  flex-shrink: 0;
  white-space: nowrap;
  font-variant-numeric: tabular-nums;
  font-size: 0.75rem;
  color: var(--text-muted);
}

.row-subject {
  margin-top: 6px;
  font-size: 0.90625rem;
  line-height: 1.35;
  color: var(--text-primary);
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.row-snippet {
  margin-top: 3px;
  font-size: 0.8125rem;
  line-height: 1.42;
  color: var(--text-muted);
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.stream-empty {
  padding: 48px 24px;
  text-align: center;
  color: var(--text-secondary);
}

.stream-empty-title {
  margin-top: 10px;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.stream-empty-desc {
  margin-top: 6px;
  margin-bottom: 16px;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.reader-empty {
  margin: auto;
  padding: 40px 24px;
  text-align: center;
  color: var(--text-muted);
}

.reader-content-wrap {
  display: flex;
  flex-direction: column;
  flex: 1 1 0%;
  min-height: 0;
}

.reader-animate #readerSubject {
  animation: readerStageIn 580ms cubic-bezier(0.16, 1, 0.3, 1) both;
}

.reader-animate .reader-meta-row {
  animation: readerStageIn 620ms cubic-bezier(0.16, 1, 0.3, 1) 70ms both;
}

.reader-animate .reader-actions {
  animation: readerStageIn 640ms cubic-bezier(0.16, 1, 0.3, 1) 130ms both;
}

.reader-animate .reader-html-frame,
.reader-animate .reader-body {
  transform-origin: top center;
  animation: readerCanvasUnfold 760ms cubic-bezier(0.16, 1, 0.3, 1) 170ms both;
}

.mobile-back-btn {
  display: none;
  margin-bottom: 12px;
  align-self: flex-start;
}

.reader-meta-row {
  margin-top: 10px;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 8px 16px;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.reader-sender-block {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}

.reader-sender-stack {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.reader-sender-strong {
  font-weight: 600;
  font-size: 0.9375rem;
  color: var(--text-primary);
  line-height: 1.25;
}

.reader-sender-sub {
  font-size: 0.78125rem;
  color: var(--text-muted);
  line-height: 1.3;
}

.reader-actions {
  flex-shrink: 0;
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 12px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle);
}

.phishing-box {
  flex-shrink: 0;
  margin-top: 10px;
  padding: 10px 14px;
  background-color: var(--bg-alert);
  border: 1px solid var(--border-danger);
  border-radius: var(--radius-base);
  color: var(--text-danger);
  font-size: 0.875rem;
  display: flex;
  align-items: center;
  gap: 10px;
}

.reader-html-frame {
  flex: 1 1 0%;
  min-height: 0;
  width: 100%;
  border: none;
  margin-top: 8px;
  background-color: var(--bg-surface);
}

.reader-body {
  flex: 1 1 0%;
  min-height: 0;
  overflow-y: auto;
  overscroll-behavior: contain;
  margin-top: 14px;
  white-space: pre-wrap;
  overflow-wrap: break-word;
  font-size: 0.96875rem;
  line-height: 1.68;
  color: var(--text-primary);
}

.toast-bar {
  position: fixed;
  right: 20px;
  bottom: 20px;
  z-index: 900;
  background-color: var(--bg-inverse);
  color: var(--text-inverse);
  padding: 10px 16px;
  border-radius: var(--radius-base);
  font-size: 0.875rem;
  font-weight: 500;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  transform: translateY(16px);
  opacity: 0;
  pointer-events: none;
  transition: transform 200ms cubic-bezier(0.22, 1, 0.36, 1), opacity 200ms ease;
}

.toast-bar.is-visible {
  transform: translateY(0);
  opacity: 1;
}

.hidden {
  display: none !important;
}

@media (max-width: 880px) {
  .shell {
    padding: 12px 14px 20px;
  }
  .search-wrap {
    order: 3;
    flex: 1 1 100%;
    max-width: none;
  }
  .inbox-grid {
    grid-template-columns: 280px minmax(0, 1fr);
    gap: 12px;
  }
  .reader-pane {
    padding: 18px 20px;
  }
}

@media (max-width: 720px) {
  .provider-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
  .onboarding-wrap {
    padding: 20px 18px;
    margin-top: 12px;
  }
  .inbox-grid {
    grid-template-columns: minmax(0, 1fr);
  }
  .mobile-back-btn {
    display: inline-flex;
  }
  .inbox-grid:not(.has-mobile-reader) .reader-pane {
    display: none;
  }
  .inbox-grid.has-mobile-reader .stream-pane {
    display: none;
  }
}

@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    transition-duration: 0.01ms !important;
    animation-duration: 0.01ms !important;
  }
}
"##;

pub const INDEX_HTML: &str = r##"<!doctype html>
<html lang="vi">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Vortex Inbox</title>
  <link rel="stylesheet" href="/tokens.css">
</head>
<body>
  <!-- Official Heroicons v2 20/solid SVG sprite (fill="currentColor") -->
  <svg class="svg-sprite" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
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
  </svg>

  <div class="shell">
    <header class="topbar">
      <span class="brand-mark">
        <svg class="hi-icon-lg" aria-hidden="true"><use href="#hi-inbox"></use></svg>
        <h1 class="brand-title">Vortex Inbox</h1>
      </span>

      <div id="topSearchWrap" class="search-wrap">
        <svg class="hi-icon" aria-hidden="true"><use href="#hi-magnifying-glass"></use></svg>
        <input id="searchInput" type="search" class="input-field" placeholder="Tìm kiếm thư, người gửi hoặc mã xác nhận...">
      </div>

      <div class="top-actions">
        <button id="btnSync" type="button" class="btn">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-arrow-path"></use></svg>
          <span data-i18n="sync_now">Làm mới</span>
        </button>
        <div id="acctSwitcher" class="acct-switcher hidden">
          <button id="btnAcctMenu" type="button" class="btn acct-trigger" aria-expanded="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-user-circle"></use></svg>
            <span id="acctActiveLabel" class="acct-active-label">Tất cả hộp thư</span>
          </button>
          <div id="acctMenu" class="acct-menu hidden" role="menu"></div>
        </div>
        <button id="btnToggleSetup" type="button" class="btn btn-primary">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-user-plus"></use></svg>
          <span id="btnToggleSetupLabel" data-i18n="connect_account">Thêm hộp thư</span>
        </button>
        <button id="btnLang" type="button" class="btn btn-ghost">
          <svg class="hi-icon" aria-hidden="true"><use href="#hi-language"></use></svg>
          <span id="langLabel">EN</span>
        </button>
      </div>
    </header>

    <!-- Instant Multi-Account OTP Banner (displays active codes across all accounts simultaneously) -->
    <section id="otpBanner" class="otp-banner hidden" aria-live="polite">
      <div id="otpBannerHead" class="otp-banner-head hidden">
        <span id="otpBannerSummary"></span>
        <div class="account-row-actions">
          <button id="btnCopyAllOtp" type="button" class="btn btn-sm btn-primary">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-clipboard-document"></use></svg>
            <span data-i18n="copy_all_codes">Sao chép tất cả</span>
          </button>
        </div>
      </div>
      <div id="otpList" class="otp-list"></div>
    </section>

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
          <button type="button" class="preset-btn" data-preset="telegram" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-user-group"></use></svg>
            <span>Telegram</span>
          </button>
          <button type="button" class="preset-btn" data-preset="rss" aria-pressed="false">
            <svg class="hi-icon" aria-hidden="true"><use href="#hi-tag"></use></svg>
            <span>RSS</span>
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
        </nav>
        <div class="filter-bar-right">
          <span id="statusNotice" class="otp-meta"></span>
          <button id="btnAutoSort" type="button" class="btn btn-sm sort-btn" aria-pressed="false">
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
      guide_telegram: "Mẹo: Nhập tên Bot và mã Bot Token lấy từ @BotFather trên Telegram.",
      guide_rss: "Mẹo: Đặt tên nguồn tin và dán đường dẫn RSS vào ô Máy chủ bên dưới.",
      guide_custom_imap: "Mẹo: Điền địa chỉ email, mật khẩu và thông tin máy chủ IMAP của bạn."
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
      guide_telegram: "Tip: Enter your Bot name and the Bot Token from @BotFather.",
      guide_rss: "Tip: Enter a name for the feed and paste the feed URL into the Server field below.",
      guide_custom_imap: "Tip: Enter your email, password, and IMAP server details below."
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
  let items = [];
  let cachedAllItems = [];
  let recentCodes = [];
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

  function cleanBodyText(subject, body) {
    let text = String(body || "").trim();
    const subj = String(subject || "").trim();
    if (!text) return "";
    if (subj) {
      const lowerSubj = subj.toLowerCase();
      while (text.toLowerCase().startsWith(lowerSubj)) {
        text = text.slice(subj.length).replace(/^[\s\u00a0—–\-:]+/, "").trim();
      }
      const dashParts = subj.split(" - ");
      if (dashParts.length > 1) {
        const tailPart = dashParts[dashParts.length - 1].trim();
        if (tailPart.length > 4 && text.toLowerCase().startsWith(tailPart.toLowerCase())) {
          text = text.slice(tailPart.length).replace(/^[\s\u00a0—–\-:]+/, "").trim();
        }
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
    } else if (preset === "telegram") {
      $("accHost").value = "api.telegram.org";
      $("accPort").value = "443";
      $("accEmail").placeholder = "@ten_bot";
    } else if (preset === "rss") {
      $("accHost").value = "https://";
      $("accPort").value = "443";
      $("accEmail").placeholder = "Tin tức";
      showServer = true;
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
    }, 520);
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

  function applyTranslations() {
    document.documentElement.lang = lang;
    $("langLabel").textContent = lang === "vi" ? "EN" : "VI";
    document.querySelectorAll("[data-i18n]").forEach((el) => {
      const k = el.getAttribute("data-i18n");
      if (I18N[lang][k]) el.textContent = I18N[lang][k];
    });
    updateSetupButtonLabel();
    $("searchInput").placeholder = t("search_placeholder");
    $("providerGuide").textContent = t("guide_" + activePreset);
    lastStreamKey = "";
    renderAccountSwitcher(cachedAllItems);
    renderStream(true);
    if (selectedId) selectItem(selectedId, false);
    renderRecentCodes();
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
    Object.keys(counts).forEach((k) => {
      const el = $("cnt_" + k);
      if (el) el.textContent = counts[k] > 0 ? String(counts[k]) : "";
    });
    requestAnimationFrame(updateCategoryGlider);
  }

  async function loadAll(forceStreamAnim = false) {
    try {
      const qs = [];
      if (activeCategory) qs.push(`category=${encodeURIComponent(activeCategory)}`);
      if (activeAccount) qs.push(`account=${encodeURIComponent(activeAccount)}`);
      const filterUrl = "/api/items" + (qs.length ? `?${qs.join("&")}` : "");

      const [status, accs, allItems, filteredList, codes] = await Promise.all([
        api("/api/status"),
        api("/api/accounts"),
        api("/api/items"),
        api(filterUrl),
        api("/api/codes")
      ]);
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
      } else if (status.missing_secret_accounts && status.missing_secret_accounts.length > 0 && allItems.length === 0) {
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
      renderStream(forceStreamAnim);
      if (items.length > 0 && (forceStreamAnim || !selectedId || !items.some((x) => x.id === selectedId))) {
        if (window.innerWidth > 720) {
          selectItem(items[0].id, false);
        }
      } else if (selectedId) {
        selectItem(selectedId, false);
      }
      renderRecentCodes();

      if (!autoEnrichedOnce && allItems.length > 0 && allItems.some((i) => !i.body_html)) {
        autoEnrichedOnce = true;
        triggerSync(null, true);
      }
    } catch (err) {
      $("statusNotice").textContent = String(err.message || err);
    }
  }

  async function triggerSync(accountId = null, silent = false) {
    if (syncInFlight) return;
    syncInFlight = true;
    if (!silent) {
      $("btnSync").classList.add("is-spinning");
      $("statusNotice").textContent = lang === "vi" ? "Đang đồng bộ song song..." : "Syncing accounts in parallel...";
    }
    try {
      const payload = accountId ? { account_id: accountId } : {};
      const res = await api("/api/sync", { method: "POST", body: JSON.stringify(payload) });
      await loadAll();
      if (!silent) {
        if (res.error) {
          $("statusNotice").textContent = lang === "vi" ? `Lỗi đồng bộ: ${res.error}` : `Sync error: ${res.error}`;
        } else {
          $("statusNotice").textContent = lang === "vi"
            ? `Đã đồng bộ (${res.synced || 0} thư mới)`
            : `Synced (${res.synced || 0} new)`;
        }
      }
    } catch (err) {
      if (!silent) {
        $("statusNotice").textContent = String(err.message || err);
      }
    } finally {
      syncInFlight = false;
      $("btnSync").classList.remove("is-spinning");
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
        triggerStreamMorph(1);
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
        triggerStreamMorph(1);
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
    const name = cleanSenderName(rawSender);
    const email = cleanSenderEmail(rawSender);
    const domain = (email.split("@")[1] || "").toLowerCase();
    const initial = ((name || email || "V").trim().charAt(0) || "V").toUpperCase();

    let primaryUrl = "";
    let secondaryUrl = "";

    if (domain === "github.com" || domain.endsWith(".github.com")) {
      const cleanHandle = name.replace(/\[bot\]$/i, "").trim();
      if (cleanHandle.toLowerCase() === "github" || email.startsWith("noreply@")) {
        primaryUrl = "https://github.com/github.png?size=64";
      } else if (/^[a-zA-Z0-9-]+$/.test(cleanHandle)) {
        primaryUrl = `https://github.com/${encodeURIComponent(cleanHandle)}.png?size=64`;
        secondaryUrl = "https://github.com/github.png?size=64";
      } else {
        primaryUrl = "https://github.com/github.png?size=64";
      }
    } else if (domain && !/^(gmail\.com|googlemail\.com|outlook\.com|hotmail\.com|icloud\.com|yahoo\.com|live\.com)$/.test(domain)) {
      primaryUrl = `https://www.google.com/s2/favicons?domain=${encodeURIComponent(domain)}&sz=64`;
      secondaryUrl = `https://icons.duckduckgo.com/ip3/${encodeURIComponent(domain)}.ico`;
    } else if (email) {
      primaryUrl = `https://unavatar.io/${encodeURIComponent(email)}?fallback=false`;
    }

    const showInitial = () => {
      avatar.innerHTML = "";
      const initSpan = document.createElement("span");
      initSpan.className = "sender-avatar-initial";
      initSpan.textContent = initial;
      avatar.appendChild(initSpan);
    };

    if (!primaryUrl) {
      showInitial();
      return avatar;
    }

    const img = document.createElement("img");
    img.className = "sender-avatar-img";
    img.src = primaryUrl;
    img.alt = name;
    img.loading = "lazy";
    img.onerror = () => {
      if (secondaryUrl && img.src !== secondaryUrl) {
        img.src = secondaryUrl;
      } else {
        showInitial();
      }
    };
    avatar.appendChild(img);
    return avatar;
  }

  function renderStream(force = false) {
    const ul = $("streamList");
    const nextKey = items.map((i) => `${i.id}:${i.state}:${i.pinned ? 1 : 0}:${(i.subject || "").length}`).join("|");
    if (!force && nextKey === lastStreamKey && ul.children.length > 0) {
      updateStreamSelectionAttr();
      return;
    }
    lastStreamKey = nextKey;
    ul.innerHTML = "";
    if (!items.length) {
      const li = document.createElement("li");
      li.className = "stream-empty";
      const icon = svgIcon("hi-inbox", "hi-icon-lg");
      const title = document.createElement("div");
      title.className = "stream-empty-title";
      title.textContent = t("empty_no_items_title");
      const desc = document.createElement("div");
      desc.className = "stream-empty-desc";
      desc.textContent = t("empty_no_items_desc");
      li.append(icon, title, desc);
      ul.appendChild(li);
      return;
    }
    items.forEach((item, idx) => {
      const li = document.createElement("li");
      li.className = "stream-row" + (item.state === "unread" ? " stream-row-unread" : "");
      li.style.setProperty("--row-idx", String(Math.min(idx, 10)));
      li.setAttribute("data-id", String(item.id));
      li.setAttribute("aria-selected", item.id === selectedId ? "true" : "false");
      li.tabIndex = 0;

      const header = document.createElement("div");
      header.className = "row-header";

      const avatar = createSenderAvatar(item.sender);

      const info = document.createElement("div");
      info.className = "sender-info";

      const topLine = document.createElement("div");
      topLine.className = "sender-top-line";

      const senderNameSpan = document.createElement("span");
      senderNameSpan.className = "row-sender-name";
      senderNameSpan.textContent = (item.pinned ? "[Ghim] " : "") + cleanSenderName(item.sender);

      const timeStr = new Date(item.received_at * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
      const timeEl = document.createElement("span");
      timeEl.className = "row-time";
      timeEl.textContent = timeStr;
      topLine.append(senderNameSpan, timeEl);

      const emailLine = document.createElement("div");
      emailLine.className = "row-sender-email";
      const sEmail = cleanSenderEmail(item.sender);
      emailLine.textContent = (!activeAccount && item.account && sEmail && sEmail !== item.account)
        ? `${sEmail} · ${item.account}`
        : (sEmail || item.account || "");

      info.append(topLine, emailLine);
      header.append(avatar, info);

      const subj = document.createElement("div");
      subj.className = "row-subject";
      subj.textContent = item.subject || "(Không tiêu đề)";

      const snip = document.createElement("div");
      snip.className = "row-snippet";
      const cleanedPreview = cleanBodyText(item.subject, item.summary || item.body_text || "");
      snip.textContent = cleanedPreview.slice(0, 140);

      li.append(header, subj, snip);
      li.onclick = () => selectItem(item.id, true);
      li.onkeydown = (e) => { if (e.key === "Enter") selectItem(item.id, true); };
      ul.appendChild(li);
    });
  }

  function updateStreamSelectionAttr() {
    document.querySelectorAll("#streamList .stream-row").forEach((li) => {
      const rowId = Number(li.getAttribute("data-id"));
      li.setAttribute("aria-selected", rowId === selectedId ? "true" : "false");
    });
  }

  function buildEmailIframeDoc(rawHtml, plainText, animate = true) {
    let bodyContent = (rawHtml || "").trim();
    if (!bodyContent) {
      const escaped = String(plainText || "")
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/(https?:\/\/[^\s"'<>)]+)/g, '<a href="$1" target="_blank" rel="noopener noreferrer">$1</a>')
        .replace(/\n/g, "<br>");
      bodyContent = `<div style="line-height:1.65;font-size:15px;">${escaped}</div>`;
    }
    const bodyAnimRule = animate
      ? "body { animation: sheetImmersion 680ms cubic-bezier(0.16, 1, 0.3, 1) both; }"
      : "";
    return `<!doctype html><html><head><meta charset="utf-8"><base target="_blank"><style>
      @keyframes sheetImmersion {
        0% {
          opacity: 0;
          transform: translate3d(0, 18px, 0) scale(0.99);
          filter: blur(4px);
        }
        65% {
          opacity: 1;
          filter: blur(0);
        }
        100% {
          opacity: 1;
          transform: translate3d(0, 0, 0) scale(1);
          filter: blur(0);
        }
      }
      html, body {
        margin: 0;
        padding: 12px 4px 28px;
        font-family: system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
        font-size: 15px;
        line-height: 1.6;
        color: #181816;
        background-color: #ffffff;
        overflow-wrap: break-word;
      }
      ${bodyAnimRule}
      img { max-width: 100%; height: auto; }
      a { color: #165640; }
      table { max-width: 100%; }
      pre, code { font-family: ui-monospace, Consolas, monospace; white-space: pre-wrap; }
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
    if (!changed && lastRenderedItemId === id && lastRenderedHtmlLen === htmlLen) {
      return;
    }
    lastRenderedItemId = id;
    lastRenderedHtmlLen = htmlLen;

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
  }

  function renderRecentCodes() {
    const nowSec = Math.floor(Date.now() / 1000);
    const active = recentCodes.filter((c) => c.expires_at > nowSec);
    const banner = $("otpBanner");
    const head = $("otpBannerHead");
    const summary = $("otpBannerSummary");
    const list = $("otpList");
    if (!banner || !list) return;

    if (active.length === 0) {
      banner.classList.add("hidden");
      list.innerHTML = "";
      return;
    }

    const ordered = activeAccount
      ? [
          ...active.filter((c) => c.account === activeAccount),
          ...active.filter((c) => c.account !== activeAccount)
        ]
      : active;
    const visible = ordered.slice(0, 8);

    banner.classList.remove("hidden");
    if (head && summary) {
      if (visible.length > 1) {
        head.classList.remove("hidden");
        summary.textContent = lang === "vi"
          ? `Đang có ${visible.length} mã OTP hoạt động từ các tài khoản`
          : `${visible.length} active OTP codes across accounts`;
        $("btnCopyAllOtp").onclick = () => {
          const text = visible
            .map((c) => `${c.account || c.service}: ${c.code}`)
            .join("\n");
          copyCode(text, true);
        };
      } else {
        head.classList.add("hidden");
      }
    }

    list.innerHTML = "";
    visible.forEach((c) => {
      const rem = Math.max(0, c.expires_at - nowSec);
      const row = document.createElement("div");
      row.className = "otp-row";

      const left = document.createElement("div");
      left.className = "otp-banner-left";

      const codeEl = document.createElement("div");
      codeEl.className = "otp-code-value";
      codeEl.textContent = c.code;

      const metaWrap = document.createElement("div");
      const accLine = document.createElement("p");
      accLine.className = "otp-account-name";
      accLine.textContent = c.account
        ? `${c.account} · ${c.service} (${cleanSenderName(c.sender)})`
        : `${c.service} (${cleanSenderName(c.sender)})`;

      const timeLine = document.createElement("p");
      timeLine.className = "otp-meta";
      timeLine.textContent = lang === "vi"
        ? `Còn hiệu lực ${Math.floor(rem / 60)} phút ${rem % 60} giây`
        : `Expires in ${Math.floor(rem / 60)}m ${rem % 60}s`;

      metaWrap.append(accLine, timeLine);
      left.append(codeEl, metaWrap);

      const right = document.createElement("div");
      right.className = "account-row-actions";

      const copyBtn = document.createElement("button");
      copyBtn.type = "button";
      copyBtn.className = "btn btn-sm btn-primary";
      copyBtn.append(
        svgIcon("hi-clipboard-document"),
        document.createTextNode(t("copy_code"))
      );
      copyBtn.onclick = () => copyCode(c.code, false, c.account);
      right.appendChild(copyBtn);

      if (c.verification_link && !c.phishing_warning) {
        const linkBtn = document.createElement("button");
        linkBtn.type = "button";
        linkBtn.className = "btn btn-sm";
        linkBtn.textContent = t("open_verify_link");
        linkBtn.onclick = () => window.open(c.verification_link, "_blank", "noopener,noreferrer");
        right.appendChild(linkBtn);
      }

      row.append(left, right);
      list.appendChild(row);
    });
  }

  async function copyCode(code, isMulti = false, accountName = "") {
    try {
      await navigator.clipboard.writeText(code);
      if (isMulti) {
        showToast(lang === "vi" ? "Đã sao chép tất cả mã OTP" : "Copied all OTP codes");
      } else {
        const suffix = accountName ? ` (${accountName})` : "";
        showToast((lang === "vi" ? `Đã sao chép mã ${code}` : `Copied code ${code}`) + suffix);
      }
      await api("/api/clipboard/copy", { method: "POST", body: JSON.stringify({ code }) });
    } catch (_) {
      showToast(code);
    }
  }

  async function runAction(itemId, actionObj) {
    if (!itemId) return;
    await api("/api/actions", {
      method: "POST",
      body: JSON.stringify({ item_id: itemId, ...actionObj })
    });
    const labels = {
      archive: lang === "vi" ? "Đã lưu trữ thư" : "Archived message",
      mark_read: lang === "vi" ? "Đã đánh dấu đã đọc" : "Marked as read",
      pin: lang === "vi" ? "Đã ghim thư" : "Pinned message",
      delete: lang === "vi" ? "Đã xóa thư" : "Deleted message"
    };
    if (labels[actionObj.action]) showToast(labels[actionObj.action]);
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

    const preset = $("accPreset").value;
    const source = preset === "telegram" ? "telegram" : (preset === "rss" ? "rss" : "email");
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

  $("btnLang").onclick = () => {
    lang = lang === "vi" ? "en" : "vi";
    applyTranslations();
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
      items = await api(`/api/items?q=${encodeURIComponent(q)}${accParam}`);
      renderStream();
    }, 180);
  };

  const catButtons = Array.from(document.querySelectorAll(".category-pills .cat-btn"));
  catButtons.forEach((btn, idx) => {
    btn.onclick = () => {
      const nextCat = btn.getAttribute("data-cat") || "";
      if (nextCat === activeCategory) return;
      const dir = idx >= prevCatIndex ? 1 : -1;
      prevCatIndex = idx;
      catButtons.forEach((b) => b.setAttribute("aria-pressed", "false"));
      btn.setAttribute("aria-pressed", "true");
      updateCategoryGlider();
      activeCategory = nextCat;
      triggerStreamMorph(dir);
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
    requestAnimationFrame(updateCategoryGlider);
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
      renderRecentCodes();
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

  setInterval(renderRecentCodes, 1000);
  selectPreset("gmail");
  loadAll();
})();
"##;

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
        assert!(STYLES_CSS.starts_with("/* hallmark"));
        assert!(!STYLES_CSS.contains("transition: all"));
        assert!(!STYLES_CSS.contains("linear-gradient"));
        assert!(!STYLES_CSS.contains("radial-gradient"));
        assert!(!STYLES_CSS.contains("100vw"));
        assert!(STYLES_CSS.contains("overflow-x: clip"));
        assert!(STYLES_CSS.contains("prefers-reduced-motion: reduce"));
        assert!(INDEX_HTML.contains("viewBox=\"0 0 20 20\" fill=\"currentColor\""));
        assert!(!INDEX_HTML.contains("fill=\"none\""));
    }
}
