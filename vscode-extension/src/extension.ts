import * as vscode from "vscode";
import { copyReference } from "ast-parser";

export function activate(context: vscode.ExtensionContext) {
  let disposable = vscode.commands.registerCommand(
    "copy-ruby-reference.copy-reference",
    () => {
      const editor = vscode.window.activeTextEditor;
      const filePath = editor?.document.fileName;
      const selection = editor?.selection;

      if (selection && filePath) {
        const line = selection.start.line + 1;
        const character = selection.start.character + 1;
        const reference = copyReference(filePath, line, character);

        if (reference !== "null") {
          vscode.env.clipboard.writeText(reference);
          vscode.window.showInformationMessage(`Copied to clipboard!`);
        } else {
          vscode.window.showInformationMessage(
            `Couldn't find a constant definition!`
          );
        }
      } else {
        vscode.window.showInformationMessage(`Invalid selection!`);
      }
    }
  );

  context.subscriptions.push(disposable);
}

export function deactivate() {}
