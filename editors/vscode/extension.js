const vscode = require('vscode');
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');

let client;

function activate(context) {
    const config = vscode.workspace.getConfiguration('mdfx');

    if (!config.get('enable', true)) {
        return;
    }

    const mdfxPath = config.get('path', 'mdfx');

    const serverOptions = {
        command: mdfxPath,
        args: ['lsp'],
        transport: TransportKind.stdio
    };

    const clientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'markdown' },
            { scheme: 'untitled', language: 'markdown' }
        ],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.md')
        }
    };

    client = new LanguageClient(
        'mdfx',
        'mdfx Language Server',
        serverOptions,
        clientOptions
    );

    client.start();

    context.subscriptions.push({
        dispose: () => {
            if (client) {
                client.stop();
            }
        }
    });

    // Register status bar item
    const statusBar = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Right,
        100
    );
    statusBar.text = '$(markdown) mdfx';
    statusBar.tooltip = 'mdfx language server active';
    statusBar.show();
    context.subscriptions.push(statusBar);

    console.log('mdfx extension activated');
}

function deactivate() {
    if (client) {
        return client.stop();
    }
    return undefined;
}

module.exports = { activate, deactivate };
