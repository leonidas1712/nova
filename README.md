<div align="center">
<h1>Nova Programming Language </h1>
</div>

![tests](https://github.com/huzaifa1712/nova/actions/workflows/tests.yml/badge.svg)


Nova is a functional programming language inspired by Lisp, Haskell, and Python.

  

- Dynamically typed

- Lazy evaluation

- Tail call optimization

- Curried and higher order functions

- Importing code

- String and list processing

- Operator precedence with `>>` (pipe), `$`, function application by spaces and parentheses

  

Example code:

```
> sum $ (ltake 3) $ (add 2) >> lmap $ lfilter even (fcons succ 10)
42
```
