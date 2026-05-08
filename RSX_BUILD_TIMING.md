# Effect of `rsx!` on Build Timing

The generated `rsx!` icon bodies were the dominant source of compile-time cost in
`dioxus-icons`, mostly because they produced a large amount of Rust for the
compiler to expand, resolve, and type-check for every generated icon.

Clean package-check timing for the `dioxus-icons` crate unit:

```text
rsx! generated icons:          23.8s
manual VDOM templates:          4.1s
manual VDOM + const builders:   2.3s
```

That is roughly a 10x reduction from the original generated `rsx!` version to
the current helper-based generated output.

Macro expansion itself was not the entire cost. In the current version, rustc
reports:

```text
total:              2.520s
macro_expand_crate: 0.212s
type_check_crate:   1.657s
generate_metadata:  0.129s
```

The bigger win came from avoiding the large expanded `rsx!` expression tree and
emitting compact release-style VDOM templates directly. That shrank the amount
of generated Rust that rustc has to resolve and type-check, which is why the
crate unit fell from about 23.8s to 2.3s.
