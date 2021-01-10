# cacheskell
This is my functional programming language that looks like Haskell but it cache every function.

# Examples
In this example we will see a recursive fibanchi sequence function. It should take a long time on most
programming languages but cacheskell will cache all function calls making it relatively fast.
```
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

```
