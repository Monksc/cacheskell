fib x
    | (equal x 0)   = 0
    | (equal x 1)   = 1
    | true          = (add (fib (sub x 1)) (fib (sub x 2)));

p x y
    | (print x) = y
    | true = y;

main
    | (p (fib 10) false) = 1
    | true = 0;

