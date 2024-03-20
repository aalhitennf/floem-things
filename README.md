
![Thonk](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRuuoecUcG7XuBfkUagFMrsRody7Lx5uG2Bv6C26Kq3pQ&s)

Some views and widgets for [Floem](https://github.com/lapce/floem)

<h4>Basic things</h4>

- Resizable and configurable Split view
- Separator
- H1/2/3/4/5/6 (text or dynamic with label)

<h4>Features behind feature flags</h4>

- <h4>async-img</h4>
    Loads image from url asynchronously on background task, requires async runtime.
    </br>
    </br>

    Enable runtime support with one of the feature flags: `tokio`, `async-std`, `smol`. Or `thread` without any async runtime.
    </br>
    Floem uses `async-std` by default so if you want to use `tokio`, disable default features on this crate.


<h4>Examples</h4>

Examples can be run with cargo.

`cargo run --example split`

`cargo run --example --no-default-features --features async-img,tokio`