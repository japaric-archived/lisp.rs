[![Build Status][status]](https://travis-ci.org/japaric/lisp.rs)

# `lisp.rs`

A lisp interpreter written in rust. See [Make-A-Lisp].

[Make-A-Lisp]: https://github.com/kanaka/mal

Status: step 3 completed

```
$ cargo build --release
$ target/release/step2
> (+ 1 2)
3
> (+ 5 (* 2 3))
11
> (def! x 3)
3
> (def! x 4)
4
> x
4
> (let* [p (+ 2 3) q (+ 2 p)] (+ p q))
12
> p
error: undefined symbol
p
^
```

# [Documentation][docs]

# License

lisp.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.

[docs]: http://japaric.github.io/lisp.rs/lisp/
[status]: https://travis-ci.org/japaric/lisp.rs.svg?branch=master
