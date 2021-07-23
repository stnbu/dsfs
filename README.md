# dsfs -- Dead Simple File Server

Someone recently complained to me that there is no<sup>[1](#is)</sup> simple cli http<sup>[2](#actually)</sup> file server for testing (and such) that you can just `crate install`.

But it made me wonder if there is no such thing (apparently) in crates.io because it is "too
obvious"... or "in the way" or...? Does not Rust need a good local development web server?

I think finding a reasonable, obvious http file server in Crates is something the Rust ecosystem
should have. If you are an experienced Rust developer, this utility is dead obvious.

But, if you are having fun checking out Rust, you might want to, say, serve a dead simple WASM
project locally without installing _Node_(!!!)

There are two dependencies and a handful of Rust code. Showing what a healthy ecosystem Rust has.
And it's very easy to understand. And it uses an immensely popular crate that has a very nice
tinkering/tinkering ratio, helping with on-boarding.

---

> <a name="is">1</a>: Granted, I did not look very hard. Relax.
> <a name="actually">2</a>: Actually, the request was for http/2. Working on it. Maybe.
