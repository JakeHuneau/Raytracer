# Raytracer

A ray tracer based on [Ray Tracing in One Weekend](https://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf).

Instructions for isntalling Rust can be found [here](https://www.rust-lang.org/tools/install)

To run this program, first compile it by running `cargo build --release`. Next, run the executable
followed by the width of the image in number of pixels followed by the height in number of pixels,
followed optionally by the number of samples taken for each pixel. For example, to produce an image
that is 2560 x 1600, call `./target/release/raytrace.exe 2560 1600`.