# ZCam -- Streaming video with zenoh
This is a simple application that shows how to stream HD Video with [zenoh](http://zenoh.io)

## Dependencies
In order to build and run ZCam you need to have [Rust](rust-lang.org) and [OpenCV]() installed on your machine. 
- [Rust](http://rust-lang.org) installation instructions are available [here](https://www.rust-lang.org/tools/install)
- [OpenCV](http://opencv.org) installation instructions are available [here](https://docs.opencv.org/trunk/df/d65/tutorial_table_of_content_introduction.html). Additional information and troubleshooting is also available [here](https://github.com/twistedfall/opencv-rust).

## Building and Running ZCam
To get and build ZCam do the following:

```
$ git clone git@github.com:kydos/zcam.git
$ cd zcam
$ cargo build --release
```

Once build you can run it as follows:

```
$ ./target/release/zdisplay -p /demo/video/yourname

$ ./target/release/zcapture -p /demo/video/yourname
```
