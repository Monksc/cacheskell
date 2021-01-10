
and l r
    | l = r
    | true = false;

or l r
    | l = true
    | true = r;

fib x
    | (equal x 0)   = 0
    | (equal x 1)   = 1
    | true          = (add (fib (sub x 1)) (fib (sub x 2)));

p x y
    | (print x) = y
    | true = y;

main
    | (print (and false false)) = 1
    | (print (and false true)) = 1
    | (print (and true false)) = 1
    | (print (and true true)) = 1

    | (print (or false false)) = 1
    | (print (or false true)) = 1
    | (print (or true false)) = 1
    | (print (or true true)) = 1

    | (print ((or true) true)) = 1

    | (print (fib 51) ) = 1
    | (print (fib 51) ) = 1
    | true = 0;

