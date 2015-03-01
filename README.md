[![Build Status][status]](https://travis-ci.org/japaric/lisp.rs)

# `lisp.rs`

A lisp interpreter written in rust. See [Make-A-Lisp].

[Make-A-Lisp]: https://github.com/kanaka/mal

Status: step 1 completed

```
$ cargo build --release
$ target/release/stage1
>   ( +   1   (+   2 3   )   )  
(+ 1 (+ 2 3))
```

# [Documentation][docs]

# License

lisp.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.

[docs]: http://japaric.github.io/lisp.rs/lisp/
[status]: https://travis-ci.org/japaric/lisp.rs.svg?branch=master
