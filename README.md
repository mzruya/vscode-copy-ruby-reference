# Copy ruby reference
A simple VSCode extension that allows you to right click on a constant definition and copy a reference to it. Unlike similar extensions, this **does not** use regular expressions and instead walks the Ruby AST, producing much more accurate results.

### How to use?
Place your caret on the desired constant definition, Then using the right click context menu select the **"Ruby: copy reference"** command. The fully qualified name of the constant will be copied to your clipboard.

![output](https://user-images.githubusercontent.com/653256/175824647-c25166d9-9dcd-4c55-9727-e3a26732a5eb.gif)

### How to install?
Go [here](https://marketplace.visualstudio.com/items?itemName=mzruya.copy-ruby-reference) and click **"Install"**.

### How to build?
Set up a [rust development environment](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repo:
```bash
> git clone https://github.com/mzruya/vscode-copy-ruby-reference.git
```

Build and package an extension for a target architecture (options are: darwin-arm64, darwin-x64, linux-x64, win32-x64) :
```bash
> cd vscode-copy-ruby-reference/vscode-extension
> npm run build-and-package-darwin-arm64
```
