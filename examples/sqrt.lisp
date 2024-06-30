(define (square x)
  (* x x))

(define (abs x)
  (if (> x 0) x (- x)))

(define (average x y)
  (/ (+ x y) 2))

(define (improve guess x)
  (average guess (/ x guess)))

(define (good-enough? old-guess guess)
  (> 0.00000001
    (/ (abs (- guess old-guess))
        old-guess)))

(define (sqrt-iter guess x)
  (if (good-enough? guess (improve guess x))
    (improve guess x)
    (sqrt-iter (improve guess x) x)))

(define (sqrt x)
  (sqrt-iter 1.0 x))



(define number 1293991231230923.1239991293)
(displayln number)
(displayln (sqrt number))
