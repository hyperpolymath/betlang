#lang racket
;; SPDX-License-Identifier: PMPL-1.0-or-later
;; betlang-to-julia.rkt - Compile betlang (Racket) to Julia
;; Copyright (C) 2026 Jonathan D.A. Jewell <jonathan.jewell@open.ac.uk>

(require racket/pretty)

(provide compile-to-julia
         compile-file-to-julia)

;; ============================================================================
;; Julia Code Generation
;; ============================================================================

(define (julia-quote value)
  "Convert Racket value to Julia literal"
  (cond
    [(string? value) (format "~s" value)]
    [(symbol? value) (format "~s" (symbol->string value))]
    [(number? value) (format "~a" value)]
    [(boolean? value) (if value "true" "false")]
    [(list? value) (format "[~a]" (string-join (map julia-quote value) ", "))]
    [else (format "~s" (~a value))]))

(define (julia-identifier sym)
  "Convert Racket identifier to Julia identifier"
  (define str (symbol->string sym))
  ;; Replace hyphens with underscores for Julia
  (string-replace str "-" "_"))

(define (compile-expr expr env)
  "Compile a single betlang expression to Julia code"
  (cond
    ;; Literals
    [(or (number? expr) (string? expr) (boolean? expr))
     (julia-quote expr)]

    ;; Symbols (variable references)
    [(symbol? expr)
     (julia-identifier expr)]

    ;; Lists (function calls)
    [(list? expr)
     (compile-form expr env)]

    [else
     (error 'compile-expr "Unsupported expression type: ~a" expr)]))

(define (compile-form form env)
  "Compile a form (list starting with operator/function)"
  (match form
    ;; (bet a b c)
    [(list 'bet a b c)
     (format "bet(~a, ~a, ~a)"
             (compile-expr a env)
             (compile-expr b env)
             (compile-expr c env))]

    ;; (bet/weighted '((a 0.5) (b 0.3) (c 0.2)))
    [(list 'bet/weighted options)
     (define opts (if (and (list? options) (eq? (car options) 'quote))
                      (cadr options)
                      options))
     (define julia-opts
       (for/list ([opt opts])
         (match opt
           [(list val weight)
            (format "(~a, ~a)" (compile-expr val env) weight)])))
     (format "bet_weighted([~a])" (string-join julia-opts ", "))]

    ;; (bet-parallel n a b c)
    [(list 'bet-parallel n a b c)
     (format "bet_parallel(~a, ~a, ~a, ~a)"
             (compile-expr n env)
             (compile-expr a env)
             (compile-expr b env)
             (compile-expr c env))]

    ;; (define var expr)
    [(list 'define var expr)
     (format "~a = ~a"
             (julia-identifier var)
             (compile-expr expr env))]

    ;; (let ([var val] ...) body)
    [(list 'let bindings body ...)
     (define binding-strs
       (for/list ([binding bindings])
         (match binding
           [(list var val)
            (format "~a = ~a"
                    (julia-identifier var)
                    (compile-expr val env))])))
     (define body-strs
       (for/list ([expr body])
         (compile-expr expr env)))
     (format "begin\n    ~a\n    ~a\nend"
             (string-join binding-strs "\n    ")
             (string-join body-strs "\n    "))]

    ;; (if test then else)
    [(list 'if test then else)
     (format "(~a ? ~a : ~a)"
             (compile-expr test env)
             (compile-expr then env)
             (compile-expr else env))]

    ;; (+ a b ...) arithmetic
    [(list '+ args ...)
     (format "(~a)" (string-join (map (λ (x) (compile-expr x env)) args) " + "))]

    [(list '- args ...)
     (format "(~a)" (string-join (map (λ (x) (compile-expr x env)) args) " - "))]

    [(list '* args ...)
     (format "(~a)" (string-join (map (λ (x) (compile-expr x env)) args) " * "))]

    [(list '/ args ...)
     (format "(~a)" (string-join (map (λ (x) (compile-expr x env)) args) " / "))]

    ;; (= a b) comparison
    [(list '= a b)
     (format "(~a == ~a)" (compile-expr a env) (compile-expr b env))]

    [(list '> a b)
     (format "(~a > ~a)" (compile-expr a env) (compile-expr b env))]

    [(list '< a b)
     (format "(~a < ~a)" (compile-expr a env) (compile-expr b env))]

    ;; (display expr) -> println
    [(list 'display expr)
     (format "println(~a)" (compile-expr expr env))]

    ;; (lambda (args ...) body) -> anonymous function
    [(list 'lambda params body ...)
     (define param-strs
       (map julia-identifier params))
     (define body-strs
       (for/list ([expr body])
         (compile-expr expr env)))
     (format "(~a) -> begin ~a end"
             (string-join param-strs ", ")
             (string-join body-strs "; "))]

    ;; Generic function call
    [(list fn args ...)
     (format "~a(~a)"
             (julia-identifier fn)
             (string-join (map (λ (x) (compile-expr x env)) args) ", "))]

    [else
     (error 'compile-form "Unsupported form: ~a" form)]))

(define (compile-to-julia forms)
  "Compile a list of betlang forms to Julia code (as string)"
  (define header
    "# SPDX-License-Identifier: PMPL-1.0-or-later\n# Generated from betlang source\n\nusing BetLang\n\n")

  (define body
    (string-join
     (for/list ([form forms])
       (compile-expr form '()))
     "\n"))

  (string-append header body "\n"))

(define (compile-file-to-julia input-file output-file)
  "Compile a betlang source file to Julia"
  (define forms (with-input-from-file input-file
                  (λ () (port->list read))))

  (define julia-code (compile-to-julia forms))

  (with-output-to-file output-file
    (λ () (display julia-code))
    #:exists 'replace)

  (printf "Compiled ~a -> ~a\n" input-file output-file))

;; ============================================================================
;; CLI Entry Point
;; ============================================================================

(module+ main
  (require racket/cmdline)

  (define input-file #f)
  (define output-file #f)

  (command-line
   #:program "betlang-to-julia"
   #:once-each
   [("-o" "--output") file "Output Julia file"
                      (set! output-file file)]
   #:args (input)
   (set! input-file input))

  (when (not input-file)
    (error "No input file specified"))

  (when (not output-file)
    ;; Default output: replace .bet/.rkt with .jl
    (set! output-file
          (path-replace-extension input-file ".jl")))

  (compile-file-to-julia input-file output-file))
