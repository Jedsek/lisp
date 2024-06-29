(define (min x y)
  (if (< x y) x y))

(define (big-sum x y z)
    (- (+ x y z)
       (min (min x y) z)))
  
(displayln (big-sum 2 3 4))
