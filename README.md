Rlisp
=====

[![Build Status](https://travis-ci.org/JoNil/rlisp.png?branch=master)](https://travis-ci.org/JoNil/rlisp)

Rlisp is a small lisp like language written in Rust. It's based on the book [Build Your Own Lisp](http://www.buildyourownlisp.com/).

Todo
====

- Change currying of lambdas to replace in the body
- Recursion
- Implement missing builins
- Add type Type
- Add literals of type Type
- Lambda argument type checking
- Add file type
- GC

Example
=======

```rust
extern crate rlisp;

fn main() {
    let mut rlisp = rlisp::Rlisp::new();

    rlisp.execute("(def {a} 12)");

    assert_eq!(rlisp.execute("(+ a a)"), "24");
}
```

Performance
===========

```
test bench_rlisp_add        ... bench:    259870 ns/iter (+/- 9592)
test bench_rlisp_add_lookup ... bench:    306329 ns/iter (+/- 5411)
test bench_rlisp_lookup     ... bench:    114719 ns/iter (+/- 2869)
```
