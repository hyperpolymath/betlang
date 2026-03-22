#lang info

;; Package metadata for betlang

(define collection "betlang")

(define version "0.1.0")

(define pkg-desc
  "A ternary probabilistic programming language for modeling uncertainty")

(define pkg-authors
  '("betlang contributors"))

(define deps
  '("base"
    "rackunit-lib"))

(define build-deps
  '("racket-doc"
    "scribble-lib"))

(define scribblings
  '())

(define test-omit-paths
  '("examples"
    "benchmarks"
    "homepage"))

(define compile-omit-paths
  '("docs"
    "examples"
    "benchmarks"
    "homepage"
    ".github"))

(define license
  'CC0-1.0)

(define categories
  '(scientific
    probabilistic-programming
    statistics
    mathematics))

(define tags
  '(probability
    statistics
    monte-carlo
    bayesian
    markov-chains
    probabilistic-programming
    ternary
    dsl))
