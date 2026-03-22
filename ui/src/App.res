// SPDX-License-Identifier: MIT OR Apache-2.0
// Betlang Web UI - Main Application
// Uses rescript-tea (The Elm Architecture)

open Tea

// ============================================================================
// Model
// ============================================================================

type ternaryValue =
  | True
  | False
  | Unknown

type outputLine =
  | Result(string)
  | Error(string)
  | Info(string)

type model = {
  code: string,
  output: array<outputLine>,
  isRunning: bool,
  history: array<string>,
  historyIndex: int,
  theme: string,
}

let init = () => {
  code: "// Welcome to Betlang!\n// Try: bet { 1, 2, 3 }\n\nlet x = bet { \"heads\", \"tails\", \"edge\" }\nx",
  output: [],
  isRunning: false,
  history: [],
  historyIndex: -1,
  theme: "dark",
}

// ============================================================================
// Messages
// ============================================================================

type msg =
  | UpdateCode(string)
  | RunCode
  | ClearOutput
  | ClearCode
  | HistoryUp
  | HistoryDown
  | SetTheme(string)
  | CodeExecuted(result<string, string>)
  | InsertExample(string)

// ============================================================================
// Update
// ============================================================================

let examples = [
  ("Ternary Bet", "let choice = bet { \"rock\", \"paper\", \"scissors\" }\nprintln(choice)"),
  ("Weighted Bet", "let result = bet { win @ 0.6, lose @ 0.3, draw @ 0.1 }\nresult"),
  ("Monte Carlo Pi", `let estimate_pi = fun n ->
  let inside = ref 0 in
  for i = 1 to n do
    let x = uniform 0.0 1.0 in
    let y = uniform 0.0 1.0 in
    if x*x + y*y <= 1.0 then inside := !inside + 1
  done;
  4.0 *. float(!inside) /. float(n)
in estimate_pi 10000`),
  ("Distribution", `let samples = replicate 100 (fun () -> normal 0.0 1.0)
let m = mean samples
let s = std samples
println("Mean: " ++ string(m))
println("Std:  " ++ string(s))`),
  ("Ternary Logic", `let a = True
let b = Unknown
let c = a && b  // Unknown
println(c)`),
]

let update = (model: model, msg: msg): (model, Cmd.t<msg>) => {
  switch msg {
  | UpdateCode(code) => ({...model, code: code}, Cmd.none)

  | RunCode => {
      let newHistory = Belt.Array.concat(model.history, [model.code])
      (
        {
          ...model,
          isRunning: true,
          history: newHistory,
          historyIndex: Belt.Array.length(newHistory),
        },
        // In a real app, this would call the WASM runtime
        Cmd.msg(CodeExecuted(Ok("=> <simulated output>")))
      )
    }

  | ClearOutput => ({...model, output: []}, Cmd.none)

  | ClearCode => ({...model, code: ""}, Cmd.none)

  | HistoryUp => {
      let newIndex = max(0, model.historyIndex - 1)
      let code = switch Belt.Array.get(model.history, newIndex) {
      | Some(c) => c
      | None => model.code
      }
      ({...model, code: code, historyIndex: newIndex}, Cmd.none)
    }

  | HistoryDown => {
      let newIndex = min(Belt.Array.length(model.history), model.historyIndex + 1)
      let code = switch Belt.Array.get(model.history, newIndex) {
      | Some(c) => c
      | None => ""
      }
      ({...model, code: code, historyIndex: newIndex}, Cmd.none)
    }

  | SetTheme(theme) => ({...model, theme: theme}, Cmd.none)

  | CodeExecuted(result) => {
      let line = switch result {
      | Ok(output) => Result(output)
      | Error(err) => Error(err)
      }
      (
        {
          ...model,
          isRunning: false,
          output: Belt.Array.concat(model.output, [line]),
        },
        Cmd.none
      )
    }

  | InsertExample(code) => ({...model, code: code}, Cmd.none)
  }
}

// ============================================================================
// View
// ============================================================================

let viewOutputLine = (line: outputLine): Vdom.t<msg> => {
  let (className, text) = switch line {
  | Result(s) => ("output-result", s)
  | Error(s) => ("output-error", s)
  | Info(s) => ("output-info", s)
  }
  Html.div([Attr.class(className)], [Html.text(text)])
}

let viewExampleButton = ((name, code): (string, string)): Vdom.t<msg> => {
  Html.button(
    [
      Attr.class("example-btn"),
      Events.onClick(InsertExample(code)),
    ],
    [Html.text(name)]
  )
}

let view = (model: model): Vdom.t<msg> => {
  Html.div(
    [Attr.class("app " ++ model.theme)],
    [
      // Header
      Html.header(
        [Attr.class("header")],
        [
          Html.h1([], [Html.text("Betlang Playground")]),
          Html.div(
            [Attr.class("theme-toggle")],
            [
              Html.button(
                [
                  Attr.class(model.theme == "dark" ? "active" : ""),
                  Events.onClick(SetTheme("dark")),
                ],
                [Html.text("Dark")]
              ),
              Html.button(
                [
                  Attr.class(model.theme == "light" ? "active" : ""),
                  Events.onClick(SetTheme("light")),
                ],
                [Html.text("Light")]
              ),
            ]
          ),
        ]
      ),

      // Main content
      Html.main(
        [Attr.class("main")],
        [
          // Examples bar
          Html.div(
            [Attr.class("examples-bar")],
            [
              Html.span([Attr.class("examples-label")], [Html.text("Examples:")]),
              Html.div(
                [Attr.class("examples-list")],
                Belt.Array.map(examples, viewExampleButton)->Belt.Array.toList,
              ),
            ]
          ),

          // Editor
          Html.div(
            [Attr.class("editor-container")],
            [
              Html.textarea(
                [
                  Attr.class("code-editor"),
                  Attr.value(model.code),
                  Attr.placeholder("Enter betlang code..."),
                  Events.onInput(s => UpdateCode(s)),
                ],
                []
              ),
            ]
          ),

          // Toolbar
          Html.div(
            [Attr.class("toolbar")],
            [
              Html.button(
                [
                  Attr.class("btn btn-primary"),
                  Attr.disabled(model.isRunning),
                  Events.onClick(RunCode),
                ],
                [Html.text(model.isRunning ? "Running..." : "Run (Ctrl+Enter)")]
              ),
              Html.button(
                [
                  Attr.class("btn btn-secondary"),
                  Events.onClick(ClearOutput),
                ],
                [Html.text("Clear Output")]
              ),
              Html.button(
                [
                  Attr.class("btn btn-secondary"),
                  Events.onClick(ClearCode),
                ],
                [Html.text("Clear Code")]
              ),
            ]
          ),

          // Output
          Html.div(
            [Attr.class("output-container")],
            [
              Html.h3([], [Html.text("Output")]),
              Html.div(
                [Attr.class("output")],
                Belt.Array.map(model.output, viewOutputLine)->Belt.Array.toList,
              ),
            ]
          ),
        ]
      ),

      // Footer
      Html.footer(
        [Attr.class("footer")],
        [
          Html.text("Betlang - A ternary probabilistic programming language"),
          Html.span([Attr.class("separator")], [Html.text(" | ")]),
          Html.a(
            [Attr.href("https://github.com/hyperpolymath/betlang")],
            [Html.text("GitHub")]
          ),
        ]
      ),
    ]
  )
}

// ============================================================================
// Subscriptions
// ============================================================================

let subscriptions = (_model: model): Sub.t<msg> => {
  Sub.none
}

// ============================================================================
// Main
// ============================================================================

let main = App.standardProgram({
  init: () => (init(), Cmd.none),
  update: update,
  view: view,
  subscriptions: subscriptions,
})
