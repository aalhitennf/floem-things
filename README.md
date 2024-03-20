
![Thonk](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRuuoecUcG7XuBfkUagFMrsRody7Lx5uG2Bv6C26Kq3pQ&s)

Some "things" for [Floem](https://github.com/lapce/floem) (git main)


<h4>Basic things</h4>

- Resizable and configurable Split view
- Separator
- H1/2/3/4/5/6 (text or dynamic with label)

<h4>Features behind feature flags</h4>

- <h4>async-img</h4>
    Loads image from url asynchronously on background task or thread.
    </br>
    </br>

    Enable runtime support with one of the feature flags: `tokio`, `async-std`, `smol`. Or `thread` without any async runtime.
    </br>
    Floem uses `async-std` by default so if you want to use `tokio`, disable default features on this crate.
- <h4>cache</h4>

    Enables `AsyncCache` that stores fetched bytes in a `DashMap`.
    </br>
    </br>

    You need to create and provide the cache with floems `provide_context` function. See `examples/async_cache.rs`.


<h4>Examples</h4>

Split</br>
`cargo run --example split`</br>

Async image</br>
`cargo run --example async_image --features async-img,{async-std,smol,thread}`</br>
`cargo run --example async_image --no-default-features --features async-img,tokio`</br>

Async image with cache</br>
`cargo run --example async_cache --features async-img,cache,{async-std,smol,thread}`</br>
`cargo run --example async_cache --no-default-features --features async-img,cache,tokio`</br>
