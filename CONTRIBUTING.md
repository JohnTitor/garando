How to do a garando release
==========================

First we need to prep the Rust repository. Check out Rust.

```
$ git clone https://github.com/rust-lang/rust
$ cd rust
```

Update the nightly version and determine it's SHA1.

```
rust$ rustup update nightly
rust$ rustc +nightly --version
rustc 1.19.0-nightly (4ed2edaaf 2017-06-01)
rust$ export RUST_SHA=4ed2edaaf
```

Check out that version.

```
rust$ git checkout $RUST_SHA
```

---

Check out garando.

```
rust$ cd ..
$ git clone https://github.com/serde-rs/garando
$ cd garando
```

Check out the `rust` branch, which tracks the upstream Rust `libsyntax`. Delete
the garando source directories and replace them with the ones from upstream.

```
garando$ git checkout origin/rust
garando$ rm -r garando_syntax/src garando_pos/src garando_errors/src
garando$ cp -r ../rust/src/libsyntax garando_syntax/src
garando$ cp -r ../rust/src/libsyntax_pos garando_pos/src
garando$ cp -r ../rust/src/librustc_errors garando_errors/src
garando$ git add .
garando$ git commit -m "Sync with $(rustc +nightly --version)"
garando$ git push origin HEAD:rust
```

Switch back to the master branch, merge it in, and resolve any conflicts.

```
garando$ git checkout origin/master
garando$ git merge origin/rust
# ... conflict resolution :-)
```

Confirm that everything compiles on stable Rust and newer.

```
garando$ cd garando_syntax
garando/garando_syntax$ cargo +stable build
garando/garando_syntax$ cargo +beta build
garando/garando_syntax$ cargo +nightly build
```

Once it works locally, push the `master` branch for CI.

```
garando$ git push origin HEAD:refs/heads/up
```

Resolve any build issues, bump the version number, tag it, and publish.

```
garando$ GIT_COMMITTER_DATE="$(git show --format=%aD | head -1)" git tag -s -m "Release 0.59.0" v0.59.0
garando$ git push origin --tags
garando$ cd garando_pos
garando/garando_pos$ cargo publish
garando/garando_pos$ cd ../garando_errors
garando/garando_errors$ cargo publish
garando/garando_errors$ cd ../garando_syntax
garando/garando_syntax$ cargo publish
```
