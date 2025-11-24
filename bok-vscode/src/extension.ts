import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {

    console.log('Congratulations, your extension "bok-vscode" is now active!');

    let disposable = vscode.commands.registerCommand('bok-vscode.helloWorld', () => {
        vscode.window.showInformationMessage('Hello World from Bok!');
    });

    context.subscriptions.push(disposable);
}

export function deactivate() {}
