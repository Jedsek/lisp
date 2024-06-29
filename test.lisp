(define (greet name)
  (string-append "Hello " name "!"))

(greet "world")

(define (fib n) (cond
  ((= n 0) 0)
  ((= n 1) 1)
  (+ (fib (- n 1)) (fib (- n 2)))
))

; (display (fib 1))
; (newline)
