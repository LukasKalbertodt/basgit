# BasGit

GitHub like web app to manage various kinds of information.
More specific information somewhat soon.
Maybe.
You can find a bit more information [in the wiki](https://github.com/LukasKalbertodt/basgit/wiki/Ideas-and-initial-notes).

## Compiling and testing

First, you have to [install `rustup`](http://rustup.rs/) to manage your Rust compilers.
After doing that and cloning this repository, change into the clone's folder and set the compiler version for this project (you only have to do this once):

```
$ rustup override set nightly
```

Now, you can simply run the application with:

```
$ cargo run
```

If compiling fails, you probably need to update your compiler version with `rustup update nightly`.

For any kind of production use, you should compile the application with `cargo run --release` and read [the Rocket guide on this topic](https://rocket.rs/guide/overview/#launching).
For development, I found it helpful to use [`watchexec`](https://github.com/mattgreen/watchexec):
install it with `cargo install watchexec` and run it with `watchexec --restart "cargo run"`.
It should automatically rebuild and start the server whenever you change a file in the directory.


---

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
