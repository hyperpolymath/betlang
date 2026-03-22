"use strict";
// SPDX-License-Identifier: MIT OR Apache-2.0
// Betlang VSCode Extension
// Thin wrapper that spawns the bet-lsp language server
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const node_1 = require("vscode-languageclient/node");
let client;
let outputChannel;
function activate(context) {
    outputChannel = vscode.window.createOutputChannel('Betlang');
    outputChannel.appendLine('Betlang extension activating...');
    // Start the language client
    startLanguageClient(context);
    // Register commands
    context.subscriptions.push(vscode.commands.registerCommand('betlang.startRepl', startRepl), vscode.commands.registerCommand('betlang.stopRepl', stopRepl), vscode.commands.registerCommand('betlang.evalSelection', evalSelection), vscode.commands.registerCommand('betlang.restartServer', () => restartServer(context)));
    outputChannel.appendLine('Betlang extension activated');
}
function startLanguageClient(context) {
    const config = vscode.workspace.getConfiguration('betlang');
    const lspPath = config.get('lspPath', 'bet-lsp');
    // Server options - spawn the LSP server
    const serverOptions = {
        run: {
            command: lspPath,
            transport: node_1.TransportKind.stdio
        },
        debug: {
            command: lspPath,
            transport: node_1.TransportKind.stdio,
            options: {
                env: { ...process.env, RUST_BACKTRACE: '1' }
            }
        }
    };
    // Client options
    const clientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'betlang' }
        ],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{bet,betlang}')
        },
        outputChannel: outputChannel,
        initializationOptions: {
            enableDiagnostics: config.get('enableDiagnostics', true),
            enableCompletion: config.get('enableCompletion', true),
            enableHover: config.get('enableHover', true)
        }
    };
    // Create and start the client
    client = new node_1.LanguageClient('betlang', 'Betlang Language Server', serverOptions, clientOptions);
    client.start().then(() => {
        outputChannel.appendLine('Language server started');
    }).catch((error) => {
        outputChannel.appendLine(`Failed to start language server: ${error}`);
        vscode.window.showErrorMessage(`Failed to start Betlang language server. Make sure 'bet-lsp' is installed and in your PATH.`);
    });
    context.subscriptions.push(client);
}
async function restartServer(context) {
    outputChannel.appendLine('Restarting language server...');
    if (client) {
        await client.stop();
    }
    startLanguageClient(context);
}
async function startRepl() {
    if (!client) {
        vscode.window.showErrorMessage('Language server not running');
        return;
    }
    try {
        const result = await client.sendRequest('betlang/repl/start', {});
        outputChannel.appendLine(`REPL started: ${JSON.stringify(result)}`);
        vscode.window.showInformationMessage('Betlang REPL started');
    }
    catch (error) {
        vscode.window.showErrorMessage(`Failed to start REPL: ${error}`);
    }
}
async function stopRepl() {
    if (!client) {
        return;
    }
    try {
        await client.sendRequest('betlang/repl/stop', {});
        outputChannel.appendLine('REPL stopped');
        vscode.window.showInformationMessage('Betlang REPL stopped');
    }
    catch (error) {
        vscode.window.showErrorMessage(`Failed to stop REPL: ${error}`);
    }
}
async function evalSelection() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || !client) {
        return;
    }
    const selection = editor.selection;
    const code = selection.isEmpty
        ? editor.document.lineAt(selection.start.line).text
        : editor.document.getText(selection);
    if (!code.trim()) {
        return;
    }
    try {
        const result = await client.sendRequest('betlang/eval', { code });
        outputChannel.appendLine(`> ${code}`);
        outputChannel.appendLine(result.result || '<no result>');
        outputChannel.show(true);
    }
    catch (error) {
        outputChannel.appendLine(`Error: ${error}`);
        outputChannel.show(true);
    }
}
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
//# sourceMappingURL=extension.js.map