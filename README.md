<div  align="center">

  

<h1>Nova Programming Language </h1>

  

</div>

  

  

![tests](https://github.com/huzaifa1712/nova/actions/workflows/tests.yml/badge.svg)

  

  

See [nova_bytecode](https://github.com/leonidas1712/nova_bytecode) for my latest attempt to rewrite Nova and make it faster.

  
  

**Note**: The full set of features is supported in the initial Python version I wrote; not all features have been ported to the Rust versions yet.

  

  

Nova is a functional programming language inspired by Lisp, Haskell, and Python.

  

  

- Dynamically typed

  

  

- Lazy evaluation

  

  

- Tail call optimization

  

  

- Curried and higher order functions

  

  

- Importing code

To be ported from Python version:

- [ ] String and list processing

  

- [ ] Operator precedence with `>>` (pipe), `$`, function application by spaces and parentheses

  

  

  

Example code:

```
>>> (def recr (x) (if (eq x 0) 0 (add x (recr (pred x)))))
recr(x) => (if (eq x 0) 0 (add x (recr (pred x))))

>>> (def app (f elem) (f elem))
app(f,elem) => (f elem)

>>> (app recr 3)
6
>>>
```

## Factorial
```
(def fac (n)
    (if (eq n 0) 1
        (let p (pred n) (mul n (fac p)))
    )
)

```

```
>>> (fac 4)
24
```
