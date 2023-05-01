# piet-winit

An attempt at using [`Piet`](github.com/linebender/piet) with `winit`, as [somebody asked](https://github.com/linebender/piet/issues/545).

This is very inefficient, as I don't understand lifetimes enough to avoid creating and dropping a new Piet bitmap canvas with every frame -- the Winit `event_loop.run` method takes a `'static` `move` closure, and it complains `borrowed value does not live long enough` / `argument requires that 'device' is borrowed for 'static` if I try creating a bitmap to reuse outside the event loop. Help would be appreciated, if this can be fixed.

Based on the [`pixels`](https://github.com/parasyte/pixels) crate raqote-winit example.

## Running

```bash
cargo run --release
```
