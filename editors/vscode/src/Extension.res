// SPDX-License-Identifier: MIT OR Apache-2.0
// Betlang VSCode Extension - ReScript Implementation
// Thin wrapper that spawns the bet-lsp language server

open VSCode

let client: ref<option<LanguageClient.t>> = ref(None)

let startLanguageClient = (context: extensionContext) => {
  let config = Workspace.getConfiguration("betlang")
  let serverPath = switch Workspace.get(config, "lspPath") {
  | Some(path) => path
  | None => "bet-lsp"
  }

  let serverOptions: LanguageClient.serverOptions = {
    run: {
      command: serverPath,
      transport: Stdio,
    },
    debug: {
      command: serverPath,
      transport: Stdio,
      options: {env: Js.Dict.fromArray([("RUST_BACKTRACE", "1")])},
    },
  }

  let clientOptions: LanguageClient.clientOptions = {
    documentSelector: [{scheme: "file", language: "betlang"}],
    synchronize: {
      fileEvents: Workspace.createFileSystemWatcher("**/*.{bet,betlang}"),
    },
  }

  let languageClient = LanguageClient.make(
    "betlang",
    "Betlang Language Server",
    serverOptions,
    clientOptions,
  )

  client := Some(languageClient)
  let _ = LanguageClient.start(languageClient)
  ()
}

let restartServer = (context: extensionContext) => {
  switch client.contents {
  | Some(c) =>
    let _ = LanguageClient.stop(c)
    startLanguageClient(context)
  | None => startLanguageClient(context)
  }
}

let activate = (context: extensionContext) => {
  startLanguageClient(context)

  // Register commands
  let restartCmd = Commands.registerCommand("betlang.restartServer", () =>
    Js.Promise.make((~resolve, ~reject as _) => {
      restartServer(context)
      resolve(.)
    })
  )
  let replStartCmd = Commands.registerCommand("betlang.startRepl", () =>
    Js.Promise.make((~resolve, ~reject as _) => {
      let _ = Window.showInformationMessage("REPL not yet implemented")
      resolve(.)
    })
  )
  let replStopCmd = Commands.registerCommand("betlang.stopRepl", () =>
    Js.Promise.make((~resolve, ~reject as _) => {
      let _ = Window.showInformationMessage("REPL not yet implemented")
      resolve(.)
    })
  )
  let evalCmd = Commands.registerCommand("betlang.evalSelection", () =>
    Js.Promise.make((~resolve, ~reject as _) => {
      let _ = Window.showInformationMessage("Eval not yet implemented")
      resolve(.)
    })
  )

  let _ = Js.Array2.push(context.subscriptions, restartCmd)
  let _ = Js.Array2.push(context.subscriptions, replStartCmd)
  let _ = Js.Array2.push(context.subscriptions, replStopCmd)
  let _ = Js.Array2.push(context.subscriptions, evalCmd)
  ()
}

let deactivate = () => {
  switch client.contents {
  | Some(c) => Some(LanguageClient.stop(c))
  | None => None
  }
}
