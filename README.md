# atcoder-202004-lock

A workspace-hack package for AtCoder.

We can easily fix the versions of crates that are available on AtCoder.
However, the versions of dependencies of dependeices are written in [`/imojudge/rust/Cargo.lock` on AtCoder](https://atcoder.jp/contests/practice/custom_test).
To gain peace of mind, you have to prepare `Cargo.lock` in every package or write versions of the 29 (= 68 - 39) packages in `dev-dependencies`.
With this package, all you have to do is to add it in the `dev-dependencies`.

## Usage

```console
❯ cargo add -D --git https://github.com/qryxip/atcoder-202004-lock atcoder-202004-lock
```

or

```toml
[dev-dependencies]
atcoder-202004-lock = { git = "https://github.com/qryxip/atcoder-202004-lock" }
```

## The versions of proconio(-derive)

The version of [proconio](https://docs.rs/crate/proconio) is `0.3.6` and the version of [proconio-derive](https://docs.rs/crate/proconio-derive) is `0.1.6`.
These versions of crates have issues, which are resolved in proconio v0.3.9 and proconio-derive v0.1.9.

- rust-analyzer cannot infer types of variables defined in `input!` from proconio v0.3.6.

    ![before](https://user-images.githubusercontent.com/14125495/111893653-8abec000-8a47-11eb-8081-37cacefd156f.png)

    With proconio v0.3.7, they are correctly infereed.

    ![after](https://user-images.githubusercontent.com/14125495/111893655-8eeadd80-8a47-11eb-8974-f5ca049f5f6b.png)

- [`#[fastout]` from proconio-derive v0.1.6 does not replace print macros in `march` arms](https://github.com/statiolake/proconio-rs/issues/8).

    ```rust
    println!("A");
    match () {
        () => println!("B"),
    }
    ```

    Under `#[fastout]` from proconio-derive v0.1.6, this code outputs:

    ```
    B
    A
    ```

    proconio-derive v0.1.9 just denies such a code.

    ```console
    error: Match arms in #[fastout] function cannot contain `print!` or `println!` macro

    note: This is because the version of proconio-derive used in the AtCoder's judge server (v0.1.6) has a bug around here. This code may not work as expected in the judge server. Sorry for inconvenience.
    For more details, see issues #8 and #14 in the repo: <https://github.com/statiolake/proconio-rs/issues/8> and <https://github.com/statiolake/proconio-rs/issues/14>.
     --> src/main.rs:5:15
      |
    5 |         () => println!("B"),
      |               ^^^^^^^

    error: aborting due to previous error

    error: could not compile `a`

    To learn more, run the command again with --verbose.
    ```

Since these fixes are not considered to affect the actual behavior, This package allows `proconio ^0.3.6` and `proconio ^0.1.6`.

## License

Licensed under [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/).
