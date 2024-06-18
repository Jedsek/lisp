;
; This is a comment
;


; Number
1.000123
0.000000
0.001
+1.123
0.123e+17
-2.123e-9

; String
"ABC"
"\n \t \b \r"
"\u0058"

; List
(1 2 3)

; Bool
#t #f

; Nil
nil

; Symbol
x y z _abc a1

; S-expr
(define (two x) (x x))
(sum (1 2 3))

; q-expr
(quote 1)
(quote (1 "s" x))
