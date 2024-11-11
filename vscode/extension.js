// @ts-check
const { LanguageClient } = require("vscode-languageclient/node");
const tmpdir = require("os").tmpdir();

module.exports = {
  /** @param {import("vscode").ExtensionContext} context*/
  activate(context) {
    /** @type {import("vscode-languageclient/node").ServerOptions} */
    const serverOptions = {
      run: {
        command: "/home/airblast/.cargo/bin/trunkls",
	args: ["-o", "/home/airblast/Desktop/text.log"]
      },
      debug: {
        command: "trunkls",
        args: [],
      },
    };

    /** @type {import("vscode-languageclient/node").LanguageClientOptions} */
    const clientOptions = {
      documentSelector: [{ scheme: "file", language: "html" }],
    };

    const client = new LanguageClient(
      "htmx-lsp",
      "Htmx Language Server",
      serverOptions,
      clientOptions
    );

    client.start();
  },
};
