# ray-tracing-in-one-weekend
Implementing the book Ray Tracing in One Weekend in Rust.

I chose Rust due to the ability to quickly multithread the rendering process.
It turned out to be quite easy to do so, and without multithreading, 
the rendering process for high resolution images would take abysmally long.

Below is a 4K render of the final image the book recommends. 
It took ~30 minutes to render on my i9 9900 with `cargo run --release`.

![render](image.png)
