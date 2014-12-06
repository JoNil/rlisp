Rlisp
=====

Rlisp is a small lisp like language written in Rust. It's based on the book [Build Your Own Lisp](http://www.buildyourownlisp.com/).

Todo
====

- Curying should work for builtins
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