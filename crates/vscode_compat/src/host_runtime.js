'use strict';

// Gram VS Code-compatible Extension Host (lightweight single shared process).
// Constraints:
// - Launched on-demand only when a dynamic .vsix activationEvent fires.
// - Runs with --max-old-space-size=256, OS low priority, and Win32 Job Object.
// - Webview is permanently unsupported; integrated terminal PTY is denied by default.

const Module = require('module');
const path = require('path');
const readline = require('readline');

const registeredCommands = new Map();
const loadedExtensions = new Map();
const docOpenListeners = [];
const docChangeListeners = [];
const docSaveListeners = [];
let statusBarSeq = 0;

function sendHostEvent(event, payload) {
  process.stdout.write(JSON.stringify({ kind: 'event', event, payload }) + '\n');
}

function sendResponse(id, result, error) {
  const msg = error ? { kind: 'response', id, error: String(error) } : { kind: 'response', id, result };
  process.stdout.write(JSON.stringify(msg) + '\n');
}

function makeEventRegistrar(list) {
  return (listener) => {
    list.push(listener);
    return {
      dispose() {
        const idx = list.indexOf(listener);
        if (idx >= 0) list.splice(idx, 1);
      },
    };
  };
}

let currentActivatingExtensionId = 'unknown';

const vscodeShim = {
  version: '1.90.0-gram-compat',
  StatusBarAlignment: { Left: 1, Right: 2 },
  DiagnosticSeverity: { Error: 0, Warning: 1, Information: 2, Hint: 3 },
  commands: {
    registerCommand(id, callback) {
      const extId = currentActivatingExtensionId;
      registeredCommands.set(id, { extId, callback });
      sendHostEvent('command.register', { extensionId: extId, id, title: id });
      return {
        dispose() {
          registeredCommands.delete(id);
        },
      };
    },
    async executeCommand(id, ...args) {
      const entry = registeredCommands.get(id);
      if (!entry) {
        throw new Error(`Command not found: ${id}`);
      }
      return await entry.callback(...args);
    },
  },
  window: {
    createStatusBarItem(alignment = 1, priority = 0) {
      const extId = currentActivatingExtensionId;
      const itemId = `sb_${++statusBarSeq}`;
      const item = {
        id: itemId,
        text: '',
        tooltip: undefined,
        command: undefined,
        alignment: alignment === 2 ? 'right' : 'left',
        priority: Number(priority) || 0,
        show() {
          sendHostEvent('ui.statusBar.set', {
            extensionId: extId,
            id: this.id,
            text: String(this.text || ''),
            tooltip: this.tooltip ? String(this.tooltip) : null,
            command: this.command ? String(this.command) : null,
            alignment: this.alignment,
            priority: this.priority,
          });
        },
        hide() {
          sendHostEvent('ui.statusBar.remove', { extensionId: extId, id: this.id });
        },
        dispose() {
          this.hide();
        },
      };
      return item;
    },
    async showInformationMessage(message) {
      sendHostEvent('ui.notify', {
        extensionId: currentActivatingExtensionId,
        level: 'info',
        message: String(message),
      });
      return undefined;
    },
    async showWarningMessage(message) {
      sendHostEvent('ui.notify', {
        extensionId: currentActivatingExtensionId,
        level: 'warning',
        message: String(message),
      });
      return undefined;
    },
    async showErrorMessage(message) {
      sendHostEvent('ui.notify', {
        extensionId: currentActivatingExtensionId,
        level: 'error',
        message: String(message),
      });
      return undefined;
    },
    async showQuickPick(items) {
      const list = Array.isArray(items) ? items : [];
      return list.length > 0 ? list[0] : undefined;
    },
    async showInputBox(options = {}) {
      return options.value ?? undefined;
    },
    // Permanently rejected: Webview (browser engine) is never adopted in Gram.
    createWebviewPanel() {
      throw new Error(
        'GRAM_UNSUPPORTED_WEBVIEW: Webview is permanently disabled in Gram to preserve low memory and GPUI performance.'
      );
    },
    registerWebviewViewProvider() {
      throw new Error(
        'GRAM_UNSUPPORTED_WEBVIEW: Webview is permanently disabled in Gram to preserve low memory and GPUI performance.'
      );
    },
    // Denied by default: integrated terminal PTY is isolated from plugins.
    createTerminal() {
      throw new Error(
        'GRAM_DENIED_TERMINAL_PTY: Integrated terminal PTY access is denied by default in Gram.'
      );
    },
    get terminals() {
      return [];
    },
  },
  workspace: {
    onDidOpenTextDocument: makeEventRegistrar(docOpenListeners),
    onDidChangeTextDocument: makeEventRegistrar(docChangeListeners),
    onDidSaveTextDocument: makeEventRegistrar(docSaveListeners),
    getConfiguration(_section) {
      return {
        get(_key, defaultValue) {
          return defaultValue;
        },
        has() {
          return false;
        },
      };
    },
  },
  languages: {
    createDiagnosticCollection(name = 'default') {
      const extId = currentActivatingExtensionId;
      return {
        name,
        set(uri, diagnostics) {
          const uriStr = typeof uri === 'string' ? uri : String(uri && uri.toString ? uri.toString() : uri);
          const list = Array.isArray(diagnostics) ? diagnostics : [];
          sendHostEvent('editor.diagnostics.publish', {
            extensionId: extId,
            uri: uriStr,
            diagnostics: list.map((d) => ({
              line: d?.range?.start?.line ?? d?.line ?? 0,
              character: d?.range?.start?.character ?? d?.character ?? 0,
              end_line: d?.range?.end?.line ?? d?.end_line ?? 0,
              end_character: d?.range?.end?.character ?? d?.end_character ?? 0,
              severity:
                d?.severity === 0
                  ? 'error'
                  : d?.severity === 1
                    ? 'warning'
                    : d?.severity === 3
                      ? 'hint'
                      : 'information',
              message: String(d?.message ?? ''),
              source: d?.source ? String(d.source) : name,
            })),
          });
        },
        clear() {
          sendHostEvent('editor.diagnostics.clear', { extensionId: extId });
        },
        dispose() {
          this.clear();
        },
      };
    },
  },
};

const origLoad = Module._load;
Module._load = function patchedLoad(request, parent, isMain) {
  if (request === 'vscode') {
    return vscodeShim;
  }
  return origLoad.apply(this, arguments);
};

async function handleRequest(req) {
  const { id, method, params = {} } = req;
  try {
    if (method === 'ping') {
      sendResponse(id, { ok: true, pid: process.pid, envKeys: Object.keys(process.env) });
      return;
    }
    if (method === 'activate') {
      const { extensionId, entryPath } = params;
      if (loadedExtensions.has(extensionId)) {
        sendResponse(id, { activated: true, cached: true });
        return;
      }
      const resolved = path.resolve(entryPath);
      const prev = currentActivatingExtensionId;
      currentActivatingExtensionId = extensionId;
      try {
        const mod = require(resolved);
        const context = { subscriptions: [] };
        if (typeof mod.activate === 'function') {
          await mod.activate(context);
        }
        loadedExtensions.set(extensionId, { mod, context });
      } finally {
        currentActivatingExtensionId = prev;
      }
      sendResponse(id, { activated: true, cached: false });
      return;
    }
    if (method === 'executeCommand') {
      const { commandId, args = [] } = params;
      const entry = registeredCommands.get(commandId);
      if (!entry) {
        throw new Error(`Command not found: ${commandId}`);
      }
      const prev = currentActivatingExtensionId;
      currentActivatingExtensionId = entry.extId;
      try {
        const result = await entry.callback(...args);
        sendResponse(id, { value: result ?? null });
      } finally {
        currentActivatingExtensionId = prev;
      }
      return;
    }
    if (method === 'notifyDocumentEvent') {
      const { eventKind, document } = params;
      const listeners =
        eventKind === 'open'
          ? docOpenListeners
          : eventKind === 'change'
            ? docChangeListeners
            : docSaveListeners;
      for (const fn of listeners) {
        await fn(document);
      }
      sendResponse(id, { notified: listeners.length });
      return;
    }
    if (method === 'shutdown') {
      for (const { mod, context } of loadedExtensions.values()) {
        for (const sub of context.subscriptions) {
          try {
            sub && typeof sub.dispose === 'function' && sub.dispose();
          } catch (_) {}
        }
        if (typeof mod.deactivate === 'function') {
          try {
            await mod.deactivate();
          } catch (_) {}
        }
      }
      sendResponse(id, { shutdown: true });
      process.exit(0);
    }
    throw new Error(`Unknown RPC method: ${method}`);
  } catch (err) {
    sendResponse(id, null, err && err.message ? err.message : String(err));
  }
}

const rl = readline.createInterface({ input: process.stdin, crlfDelay: Infinity });
rl.on('line', (line) => {
  const trimmed = line.trim();
  if (!trimmed) return;
  let msg;
  try {
    msg = JSON.parse(trimmed);
  } catch (err) {
    sendResponse(0, null, `Invalid JSON: ${err.message}`);
    return;
  }
  handleRequest(msg);
});

rl.on('close', () => {
  process.exit(0);
});
