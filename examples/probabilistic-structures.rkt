#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/combinators.rkt")

;; Probabilistic Data Structures with betlang

(displayln "=== Probabilistic Data Structures ===\n")

;; 1. Skip List (simplified)
(displayln "1. Probabilistic Skip List")
(struct skip-node (value next down) #:transparent)

(define (random-level max-level)
  ;; Geometric distribution for level
  (let loop ([level 0])
    (if (or (>= level max-level) (= (bet 0 1 0) 1))
        level
        (loop (+ level 1)))))

(define levels
  (for/list ([i (in-range 20)])
    (random-level 3)))
(displayln (format "   Random levels for skip list: ~a" levels))
(displayln (format "   Average level: ~a" (exact->inexact (mean levels))))

;; 2. Bloom Filter
(displayln "\n2. Bloom Filter (probabilistic membership)")
(struct bloom-filter (size bits hash-count) #:mutable #:transparent)

(define (make-bloom-filter size hash-count)
  (bloom-filter size (make-vector size #f) hash-count))

(define (hash-value x seed)
  (modulo (+ (* x seed) seed) (bloom-filter-size (make-bloom-filter 100 3))))

(define (bloom-add! bf x)
  (for ([h (in-range (bloom-filter-hash-count bf))])
    (define idx (modulo (+ (* x h) h) (bloom-filter-size bf)))
    (vector-set! (bloom-filter-bits bf) idx #t)))

(define (bloom-contains? bf x)
  (for/and ([h (in-range (bloom-filter-hash-count bf))])
    (define idx (modulo (+ (* x h) h) (bloom-filter-size bf)))
    (vector-ref (bloom-filter-bits bf) idx)))

(define bf (make-bloom-filter 100 3))
(for ([x '(1 5 10 15 20)])
  (bloom-add! bf x))

(displayln "   Added elements: 1, 5, 10, 15, 20")
(displayln (format "     Contains 5? ~a (true positive)" (bloom-contains? bf 5)))
(displayln (format "     Contains 7? ~a (false positive check)" (bloom-contains? bf 7)))
(displayln (format "     Contains 15? ~a (true positive)" (bloom-contains? bf 15)))

;; Test false positive rate
(define false-positives
  (count identity
         (for/list ([x (in-range 100 200)])
           (bloom-contains? bf x))))
(displayln (format "     False positive rate: ~a%" false-positives))

;; 3. HyperLogLog (cardinality estimation)
(displayln "\n3. HyperLogLog Cardinality Estimation (simplified)")

(define (leading-zeros n)
  (if (= n 0)
      32
      (let loop ([x n] [count 0])
        (if (= (bitwise-and x 1) 1)
            count
            (loop (arithmetic-shift x -1) (+ count 1))))))

(define (hll-hash x)
  (modulo (* x 2654435761) (expt 2 32)))

(define (hll-estimate values m)
  ;; m is number of registers
  (define registers (make-vector m 0))
  (for ([v values])
    (define h (hll-hash v))
    (define reg-idx (modulo h m))
    (define lz (leading-zeros (quotient h m)))
    (vector-set! registers reg-idx (max (vector-ref registers reg-idx) lz)))

  (define raw-estimate
    (/ (* m m)
       (for/sum ([r registers])
         (expt 2 (- r)))))
  raw-estimate)

(define test-set (for/list ([i (in-range 1000)]) (random 10000)))
(define actual-cardinality (length (remove-duplicates test-set)))
(define estimated-cardinality (hll-estimate test-set 64))
(displayln (format "   Actual unique elements: ~a" actual-cardinality))
(displayln (format "   Estimated unique elements: ~a" (exact->inexact estimated-cardinality)))
(displayln (format "   Error: ~a%"
                   (exact->inexact
                    (* 100 (/ (abs (- estimated-cardinality actual-cardinality))
                             actual-cardinality)))))

;; 4. Count-Min Sketch
(displayln "\n4. Count-Min Sketch (frequency estimation)")
(struct count-min (width depth table) #:transparent)

(define (make-count-min width depth)
  (count-min width depth
             (for/list ([d (in-range depth)])
               (make-vector width 0))))

(define (cm-hash x seed width)
  (modulo (+ (* x seed) seed) width))

(define (cm-update! cm x count)
  (define w (count-min-width cm))
  (for ([d (in-range (count-min-depth cm))]
        [row (count-min-table cm)])
    (define idx (cm-hash x d w))
    (vector-set! row idx (+ (vector-ref row idx) count))))

(define (cm-query cm x)
  (define w (count-min-width cm))
  (apply min
         (for/list ([d (in-range (count-min-depth cm))]
                    [row (count-min-table cm)])
           (vector-ref row (cm-hash x d w)))))

(define cm (make-count-min 100 5))
(define stream (for/list ([i (in-range 10000)])
                 (bet 1 2 3)))

(for ([x stream])
  (cm-update! cm x 1))

(displayln "   Processed 10000 items from stream (bet 1 2 3)")
(for ([x '(1 2 3)])
  (define actual (count (lambda (y) (= y x)) stream))
  (define estimated (cm-query cm x))
  (displayln (format "     Frequency of ~a: actual=~a, estimated=~a"
                     x actual estimated)))

;; 5. Reservoir Sampling
(displayln "\n5. Reservoir Sampling (random sample from stream)")

(define (reservoir-sample stream k)
  (define reservoir (make-vector k #f))
  ;; Initialize with first k elements
  (for ([i (in-range (min k (length stream)))])
    (vector-set! reservoir i (list-ref stream i)))

  ;; Process remaining elements
  (for ([i (in-range k (length stream))])
    (define j (random (+ i 1)))
    (when (< j k)
      (vector-set! reservoir j (list-ref stream i))))

  (vector->list reservoir))

(define large-stream (for/list ([i (in-range 10000)]) i))
(define sample (reservoir-sample large-stream 10))
(displayln (format "   Sample of 10 from stream of 10000: ~a" sample))

;; Verify uniform distribution
(define samples-of-10
  (for/list ([trial (in-range 1000)])
    (car (reservoir-sample '(0 1 2 3 4 5 6 7 8 9) 1))))
(define sample-freq (frequency-table samples-of-10))
(displayln "   Distribution of 1000 samples (should be uniform):")
(for ([pair (sort sample-freq < #:key car)])
  (displayln (format "     ~a: ~a times" (car pair) (cdr pair))))

;; 6. Probabilistic Hash Table
(displayln "\n6. Probabilistic Hash Table with Cuckoo Hashing")

(define (cuckoo-hash1 x size) (modulo (* x 2654435761) size))
(define (cuckoo-hash2 x size) (modulo (* x 2147483647) size))

(define (cuckoo-insert table x max-attempts)
  (define size (vector-length table))
  (let loop ([item x] [attempts 0] [use-hash1 #t])
    (cond
      [(>= attempts max-attempts) #f] ;; Failed to insert
      [else
       (define idx (if use-hash1
                       (cuckoo-hash1 item size)
                       (cuckoo-hash2 item size)))
       (define current (vector-ref table idx))
       (cond
         [(not current) ;; Empty slot
          (vector-set! table idx item)
          #t]
         [else ;; Displace current item
          (vector-set! table idx item)
          (loop current (+ attempts 1) (not use-hash1))])])))

(define cuckoo-table (make-vector 20 #f))
(define items '(42 17 88 93 15 7 23 19 51 64))
(displayln (format "   Inserting items: ~a" items))
(for ([x items])
  (cuckoo-insert cuckoo-table x 100))
(displayln (format "   Table state (some slots may be empty): ~a"
                   (filter identity (vector->list cuckoo-table))))

;; 7. Random Treap
(displayln "\n7. Treap (Tree + Heap with random priorities)")
(struct treap-node (key priority left right) #:transparent)

(define (random-priority) (random 1000))

(define (treap-insert node key)
  (define new-priority (random-priority))
  (if (not node)
      (treap-node key new-priority #f #f)
      (if (< key (treap-node-key node))
          (treap-node (treap-node-key node)
                      (treap-node-priority node)
                      (treap-insert (treap-node-left node) key)
                      (treap-node-right node))
          (treap-node (treap-node-key node)
                      (treap-node-priority node)
                      (treap-node-left node)
                      (treap-insert (treap-node-right node) key)))))

(define (treap-height node)
  (if (not node)
      0
      (+ 1 (max (treap-height (treap-node-left node))
                (treap-height (treap-node-right node))))))

(define my-treap
  (for/fold ([t #f])
            ([x '(5 3 7 2 4 6 8 1 9)])
    (treap-insert t x)))

(displayln (format "   Inserted keys: 5 3 7 2 4 6 8 1 9"))
(displayln (format "   Treap height: ~a (expected ~log₂(9) ≈ 3-4)"
                   (treap-height my-treap)))

(displayln "\n=== Probabilistic Data Structures Complete ===")
