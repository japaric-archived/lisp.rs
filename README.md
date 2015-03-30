[![Build Status][status]](https://travis-ci.org/japaric/lisp.rs)

# `lisp.rs`

A lisp interpreter written in rust. See [Make-A-Lisp].

[Make-A-Lisp]: https://github.com/kanaka/mal

Status: step 2 completed

```
$ cargo build --release
$ target/release/step2
> (+ 1 2)
3
> (+ 5 (* 2 3))
11
> (abc 1 2)
error: undefined symbol
(abc 1 2)
 ^~~
> (1 2 3)
error: expected symbol
(1 2 3)
 ^
```

# [Documentation][docs]

# License

lisp.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.

[docs]: http://japaric.github.io/lisp.rs/lisp/
[status]: https://travis-ci.org/japaric/lisp.rs.svg?branch=master
