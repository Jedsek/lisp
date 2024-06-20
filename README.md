# Lisp 

(WIP)

Type `cargo r -p repl` in workspace root dir.

- Examples:

```lisp
; 15
(+ 1 2 3 4 5) 

; 1.1111
(+ 1 1e-1 1e-2 1e-3 1e-4)

; #f
(> 1 2 3 4 5)

; #t
(= 1 1 1 1 1)


; Fibonacci sequence
; Calculate it with the recursive way
(def fib (fn n (
  ; If n == 0 or n == 1, return 
  if (= n 0) 0 (
  if (= n 1) 1 (

  ; If n > 1, return `fib(n - 1) + fib(n - 2)`
  (+ (fib (- n 1))
     (fib (- n 2))))
))))

; Print the result with newline
(println (fib 10))
```

# Showcase

![show](screenshots/1.png)
