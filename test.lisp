1
213
((1))
(= 1 2 3)
(+ 1 -2)
(- (+ 2 3) (* 2 2 2 2 2))
(/ (+ 2 3) (* 1 2 3))
(+ 1 1e-1 1e-2 1e-3)


; Fibonacci sequence
; Calculate it with the recursive way
(def fib (fn n (
	; If n == 0 or n == 1, return 
	if (= n 0) 0 (
	if (= n 1) 1 (

	; If n > 1, return `fib(n - 1) + fib(n - 2)`
	(+ (fib (- n 1))
		 (fib (- n 2)))	)
))))

; Print the result with newline
(println (fib 10))
