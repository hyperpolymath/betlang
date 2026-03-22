#lang racket
;; SPDX-License-Identifier: PMPL-1.0-or-later
;; Cool-Off Mechanism
;;
;; Time-locked betting prevention to reduce compulsive gambling.
;; Enforces mandatory delays between bets to promote responsible gambling.

(provide make-cool-off-tracker
         bet/cool-off
         cool-off-active?
         time-until-next-bet
         reset-cool-off
         cool-off-violation-count
         set-cool-off-period!
         get-cool-off-period
         cool-off-enabled?
         enable-cool-off!
         disable-cool-off!
         bet-history
         session-stats)

;; Cool-off state (mutable for session tracking)
(struct cool-off-state
  (last-bet-time        ;; timestamp of last bet
   cool-off-period      ;; seconds required between bets
   enabled              ;; whether cool-off is active
   violation-count      ;; number of attempted violations
   bet-history)         ;; list of bet timestamps
  #:mutable
  #:transparent)

;; Global cool-off tracker (can be overridden per-session)
(define default-cool-off
  (cool-off-state (current-seconds) 0 #f 0 '()))

;; Create a new cool-off tracker
(define (make-cool-off-tracker cool-off-seconds [enabled #t])
  (cool-off-state 0 cool-off-seconds enabled 0 '()))

;; Check if cool-off period is active
(define (cool-off-active? tracker)
  (and (cool-off-state-enabled tracker)
       (> (cool-off-state-cool-off-period tracker) 0)
       (let ([elapsed (- (current-seconds) (cool-off-state-last-bet-time tracker))])
         (< elapsed (cool-off-state-cool-off-period tracker)))))

;; Get time remaining until next bet allowed (in seconds)
(define (time-until-next-bet tracker)
  (if (cool-off-active? tracker)
      (- (cool-off-state-cool-off-period tracker)
         (- (current-seconds) (cool-off-state-last-bet-time tracker)))
      0))

;; Execute a bet with cool-off enforcement
(define (bet/cool-off tracker bet-fn . args)
  (when (cool-off-active? tracker)
    (define remaining (time-until-next-bet tracker))
    ;; Increment violation count
    (set-cool-off-state-violation-count!
     tracker
     (+ (cool-off-state-violation-count tracker) 1))

    (error 'bet/cool-off
           "Cool-off period active. Please wait ~a seconds before placing another bet. (Violations: ~a)"
           (ceiling remaining)
           (cool-off-state-violation-count tracker)))

  ;; Record bet time
  (set-cool-off-state-last-bet-time! tracker (current-seconds))
  (set-cool-off-state-bet-history!
   tracker
   (cons (current-seconds) (cool-off-state-bet-history tracker)))

  ;; Execute the bet
  (apply bet-fn args))

;; Reset cool-off timer (for testing or administrative override)
(define (reset-cool-off tracker)
  (set-cool-off-state-last-bet-time! tracker 0)
  (set-cool-off-state-violation-count! tracker 0)
  (set-cool-off-state-bet-history! tracker '()))

;; Get violation count
(define (cool-off-violation-count tracker)
  (cool-off-state-violation-count tracker))

;; Set cool-off period (in seconds)
(define (set-cool-off-period! tracker seconds)
  (set-cool-off-state-cool-off-period! tracker seconds))

;; Get current cool-off period
(define (get-cool-off-period tracker)
  (cool-off-state-cool-off-period tracker))

;; Enable/disable cool-off
(define (enable-cool-off! tracker)
  (set-cool-off-state-enabled! tracker #t))

(define (disable-cool-off! tracker)
  (set-cool-off-state-enabled! tracker #f))

(define (cool-off-enabled? tracker)
  (cool-off-state-enabled tracker))

;; Get bet history
(define (bet-history tracker)
  (reverse (cool-off-state-bet-history tracker)))

;; Calculate session statistics
(define (session-stats tracker)
  (define history (cool-off-state-bet-history tracker))
  (if (empty? history)
      (hash 'total-bets 0
            'session-duration 0
            'avg-bet-interval 0
            'violations (cool-off-state-violation-count tracker))
      (let* ([total-bets (length history)]
             [first-bet (last history)]
             [last-bet (first history)]
             [duration (- last-bet first-bet)]
             [intervals (for/list ([t1 (rest history)]
                                  [t2 history])
                         (- t1 t2))]
             [avg-interval (if (empty? intervals)
                              0
                              (/ (apply + intervals) (length intervals)))])
        (hash 'total-bets total-bets
              'session-duration duration
              'avg-bet-interval avg-interval
              'violations (cool-off-state-violation-count tracker)
              'bets-per-minute (if (zero? duration)
                                  0
                                  (/ total-bets (/ duration 60.0)))))))

;; Adaptive cool-off: increase period based on rapid betting
(define (adaptive-cool-off-period tracker base-period)
  (define stats (session-stats tracker))
  (define bpm (hash-ref stats 'bets-per-minute 0))
  (cond
    [(> bpm 10) (* base-period 4)]  ;; Very rapid betting - 4x cool-off
    [(> bpm 5) (* base-period 2)]   ;; Rapid betting - 2x cool-off
    [else base-period]))             ;; Normal pace

;; Self-exclusion: permanent cool-off for a duration
(struct self-exclusion
  (start-time
   duration-seconds
   reason)
  #:transparent)

(define (make-self-exclusion duration-days reason)
  (self-exclusion (current-seconds)
                 (* duration-days 24 60 60)
                 reason))

(define (self-exclusion-active? exclusion)
  (< (- (current-seconds) (self-exclusion-start-time exclusion))
     (self-exclusion-duration-seconds exclusion)))

(define (time-remaining exclusion)
  (max 0 (- (self-exclusion-duration-seconds exclusion)
            (- (current-seconds) (self-exclusion-start-time exclusion)))))

;; Example usage and tests
(module+ test
  (require rackunit)

  (test-case "Cool-off basic functionality"
    (define tracker (make-cool-off-tracker 2 #t))

    ;; First bet should succeed
    (check-not-exn
     (lambda () (bet/cool-off tracker (lambda (x) x) 42)))

    ;; Immediate second bet should fail
    (check-exn exn:fail?
               (lambda () (bet/cool-off tracker (lambda (x) x) 42)))

    ;; Check violation count
    (check-equal? (cool-off-violation-count tracker) 1))

  (test-case "Cool-off period expiry"
    (define tracker (make-cool-off-tracker 1 #t))

    (bet/cool-off tracker (lambda (x) x) 1)
    (sleep 1.1)  ;; Wait for cool-off to expire

    ;; Should succeed after waiting
    (check-not-exn
     (lambda () (bet/cool-off tracker (lambda (x) x) 2))))

  (test-case "Disable cool-off"
    (define tracker (make-cool-off-tracker 10 #t))

    (bet/cool-off tracker (lambda (x) x) 1)
    (disable-cool-off! tracker)

    ;; Should succeed immediately when disabled
    (check-not-exn
     (lambda () (bet/cool-off tracker (lambda (x) x) 2)))))

(module+ main
  (displayln "=== Cool-Off Mechanism Examples ===\n")

  (displayln "1. Basic cool-off (5 second period):")
  (define tracker (make-cool-off-tracker 5 #t))

  (displayln "   Placing first bet...")
  (bet/cool-off tracker (lambda (x) (displayln (format "   Result: ~a" x))) "Win")

  (displayln "\n   Attempting immediate second bet...")
  (with-handlers ([exn:fail? (lambda (e)
                              (displayln (format "   Blocked: ~a" (exn-message e))))])
    (bet/cool-off tracker (lambda (x) x) "Win"))

  (displayln (format "\n   Time until next bet allowed: ~a seconds"
                    (ceiling (time-until-next-bet tracker))))

  (displayln "\n2. Session statistics:")
  (define tracker2 (make-cool-off-tracker 0 #f))  ;; Disabled for demo
  (for ([i 5])
    (bet/cool-off tracker2 (lambda (x) x) i)
    (sleep 0.5))

  (define stats (session-stats tracker2))
  (displayln (format "   Total bets: ~a" (hash-ref stats 'total-bets)))
  (displayln (format "   Session duration: ~a seconds"
                    (inexact->exact (ceiling (hash-ref stats 'session-duration)))))
  (displayln (format "   Average interval: ~a seconds"
                    (hash-ref stats 'avg-bet-interval)))
  (displayln (format "   Bets per minute: ~a"
                    (hash-ref stats 'bets-per-minute)))

  (displayln "\n3. Self-exclusion:")
  (define exclusion (make-self-exclusion 30 "Voluntary 30-day break"))
  (displayln (format "   Active: ~a" (self-exclusion-active? exclusion)))
  (displayln (format "   Time remaining: ~a days"
                    (/ (time-remaining exclusion) 86400))))
