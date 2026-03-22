#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")

;; Game Theory Examples using betlang

(displayln "=== Game Theory with betlang ===\n")

;; Example 1: Rock-Paper-Scissors
(displayln "1. Rock-Paper-Scissors:")
(define (rps-payoff player1 player2)
  (match (list player1 player2)
    [(list 'rock 'scissors) 1]
    [(list 'scissors 'paper) 1]
    [(list 'paper 'rock) 1]
    [(list a b) (if (equal? a b) 0 -1)]))

(define (play-rps strategy1 strategy2 n)
  (for/sum ([i (in-range n)])
    (rps-payoff (strategy1) (strategy2))))

(define (random-rps) (bet 'rock 'paper 'scissors))
(define (always-rock) 'rock)
(define (weighted-rps) (bet/weighted '(rock 2) '(paper 2) '(scissors 1)))

(define random-vs-random (play-rps random-rps random-rps 1000))
(define rock-vs-random (play-rps always-rock random-rps 1000))
(define weighted-vs-random (play-rps weighted-rps random-rps 1000))

(displayln (format "   Random vs Random: ~a" random-vs-random))
(displayln (format "   Always Rock vs Random: ~a" rock-vs-random))
(displayln (format "   Weighted vs Random: ~a" weighted-vs-random))

;; Example 2: Prisoner's Dilemma
(displayln "\n2. Prisoner's Dilemma:")
(define (prisoners-dilemma p1 p2)
  (match (list p1 p2)
    [(list 'cooperate 'cooperate) '(3 3)]
    [(list 'cooperate 'defect) '(0 5)]
    [(list 'defect 'cooperate) '(5 0)]
    [(list 'defect 'defect) '(1 1)]))

(define (always-cooperate) 'cooperate)
(define (always-defect) 'defect)
(define (tit-for-tat last-opponent-move)
  (if (equal? last-opponent-move 'defect)
      'defect
      'cooperate))
(define (random-strategy) (bet 'cooperate 'defect 'cooperate))

(define (play-iterated-pd strategy1 strategy2 rounds)
  (let loop ([r rounds] [s1-total 0] [s2-total 0]
             [s1-last 'cooperate] [s2-last 'cooperate])
    (if (<= r 0)
        (list s1-total s2-total)
        (let* ([move1 (strategy1 s2-last)]
               [move2 (strategy2 s1-last)]
               [payoffs (prisoners-dilemma move1 move2)])
          (loop (- r 1)
                (+ s1-total (first payoffs))
                (+ s2-total (second payoffs))
                move1
                move2)))))

(define coop-vs-defect (play-iterated-pd
                        (lambda (x) (always-cooperate))
                        (lambda (x) (always-defect))
                        100))
(displayln (format "   Cooperate vs Defect (100 rounds): ~a" coop-vs-defect))

(define tft-vs-random (play-iterated-pd
                       tit-for-tat
                       (lambda (x) (random-strategy))
                       100))
(displayln (format "   Tit-for-Tat vs Random (100 rounds): ~a" tft-vs-random))

;; Example 3: Matching Pennies
(displayln "\n3. Matching Pennies:")
(define (matching-pennies p1 p2)
  (if (equal? p1 p2)
      '(1 -1)  ;; Player 1 wins
      '(-1 1))) ;; Player 2 wins

(define (random-coin) (bet 'heads 'tails 'heads))
(define (biased-coin) (bet/weighted '(heads 3) '(tails 2) '(heads 1)))

(define (play-matching n s1 s2)
  (for/fold ([p1-score 0] [p2-score 0])
            ([i (in-range n)])
    (define result (matching-pennies (s1) (s2)))
    (values (+ p1-score (first result))
            (+ p2-score (second result)))))

(define-values (mp-p1 mp-p2) (play-matching 1000 random-coin random-coin))
(displayln (format "   Random vs Random: P1=~a, P2=~a" mp-p1 mp-p2))

(define-values (mp-b1 mp-b2) (play-matching 1000 biased-coin random-coin))
(displayln (format "   Biased vs Random: P1=~a, P2=~a" mp-b1 mp-b2))

;; Example 4: Battle of the Sexes
(displayln "\n4. Battle of the Sexes:")
(define (battle-of-sexes p1 p2)
  (match (list p1 p2)
    [(list 'opera 'opera) '(2 1)]
    [(list 'football 'football) '(1 2)]
    [_ '(0 0)]))

(define (prefer-opera) (bet 'opera 'opera 'football))
(define (prefer-football) (bet 'football 'football 'opera))
(define (random-choice) (bet 'opera 'football 'opera))

(define (play-battle n s1 s2)
  (for/fold ([p1-score 0] [p2-score 0])
            ([i (in-range n)])
    (define result (battle-of-sexes (s1) (s2)))
    (values (+ p1-score (first result))
            (+ p2-score (second result)))))

(define-values (bs-p1 bs-p2) (play-battle 1000 prefer-opera prefer-football))
(displayln (format "   Opera-lover vs Football-lover: P1=~a, P2=~a" bs-p1 bs-p2))

(define-values (bs-r1 bs-r2) (play-battle 1000 random-choice random-choice))
(displayln (format "   Random vs Random: P1=~a, P2=~a" bs-r1 bs-r2))

;; Example 5: Evolutionary Game Theory - Hawk-Dove
(displayln "\n5. Hawk-Dove Game (Evolutionary):")
(define V 50)  ;; Value of resource
(define C 100) ;; Cost of fighting

(define (hawk-dove p1 p2)
  (match (list p1 p2)
    [(list 'hawk 'hawk) (list (/ (- V C) 2) (/ (- V C) 2))]
    [(list 'hawk 'dove) (list V 0)]
    [(list 'dove 'hawk) (list 0 V)]
    [(list 'dove 'dove) (list (/ V 2) (/ V 2))]))

;; Population simulation
(define (evolve-population pop-hawks pop-doves generations)
  (define total (+ pop-hawks pop-doves))
  (define p-hawk (/ pop-hawks total))
  (define p-dove (/ pop-doves total))

  (displayln (format "   Generation 0: Hawks=~a%, Doves=~a%"
                     (* 100 p-hawk) (* 100 p-dove)))

  (for/fold ([hawks pop-hawks] [doves pop-doves])
            ([gen (in-range generations)])
    (define total-pop (+ hawks doves))
    (define hawk-fitness
      (+ (* (/ hawks total-pop) (first (hawk-dove 'hawk 'hawk)))
         (* (/ doves total-pop) (first (hawk-dove 'hawk 'dove)))))
    (define dove-fitness
      (+ (* (/ hawks total-pop) (first (hawk-dove 'dove 'hawk)))
         (* (/ doves total-pop) (first (hawk-dove 'dove 'dove)))))

    (define new-total (* total-pop 1.1)) ;; Population growth
    (define new-hawks (exact->inexact (* new-total (/ hawk-fitness (+ hawk-fitness dove-fitness)))))
    (define new-doves (- new-total new-hawks))

    (when (= (modulo (+ gen 1) 25) 0)
      (displayln (format "   Generation ~a: Hawks=~a%, Doves=~a%"
                         (+ gen 1)
                         (* 100 (/ new-hawks new-total))
                         (* 100 (/ new-doves new-total)))))

    (values new-hawks new-doves)))

(evolve-population 500 500 100)

;; Example 6: Auction Theory - All-pay auction
(displayln "\n6. All-Pay Auction:")
(define (all-pay-auction bids)
  (define max-bid (apply max bids))
  (define winner-idx (index-of bids max-bid))
  (for/list ([bid bids] [idx (in-naturals)])
    (if (= idx winner-idx)
        (- 100 bid) ;; Winner gets value minus bid
        (- bid))))  ;; Losers just pay their bid

(define (random-bid) (bet 10 30 50))
(define (conservative-bid) (bet 5 10 15))
(define (aggressive-bid) (bet 40 60 80))

(define auction-results
  (for/list ([i (in-range 100)])
    (all-pay-auction (list (random-bid) (conservative-bid) (aggressive-bid)))))

(displayln (format "   Average payoffs over 100 auctions:"))
(displayln (format "     Random bidder: ~a"
                   (mean (map first auction-results))))
(displayln (format "     Conservative bidder: ~a"
                   (mean (map second auction-results))))
(displayln (format "     Aggressive bidder: ~a"
                   (mean (map third auction-results))))

(displayln "\n=== Game Theory Examples Complete ===")
