# Hello WasmEdge

Please [install the prerequisites](../README.md) first!

## Quick start with Docker

```
$ docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-hello:latest
Hello WasmEdge!
```

## Code

The [`src/main.rs`](src/main.rs) source code shows

* A standalone Rust app must have a `main()` function as the entry point.
* In the `main()` function, we create a string. The string is `&str` type. In Rust, it is called a string slice meaning that this string is immutable.
* We print the string using the `println!()` marco. The `!` indicates that it is a macro, which is a set of functions to perform the task of printing to the OS console. The Rust compile expands the macro into a set of functions at compile time. There are many such macros in Rust and it is crucial that you learn them!

## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi --release
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge target/wasm32-wasi/release/hello.wasm
Hello WasmEdge!
```

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub. The process is slightly different depending on how you plan to use the image.

### For Docker Desktop and containerd

For containerd based systems, such as the Docker Desktop and many flavors of Kubernetes, 
you just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ docker buildx build --platform wasi/wasm -t secondstate/rust-example-hello .
... ...
$ docker push secondstate/rust-example-hello
```

### For Podman, OpenShift and CRI-O

For `crun` based systems, such as Podman, OpenShift and CRI-O, 
you will need to specify a special annotation for the image so that `crun` knowns to use WasmEdge to run it.
In this example, we push the image to Docker Hub with a `crun` tag.

```
$ sudo buildah build --annotation "module.wasm.image/variant=compat-smart" -t rust-example-hello .
#
# make sure docker is install and running
# systemctl status docker
# to make sure regular user can use docker
# sudo usermod -aG docker $USER#
# newgrp docker

# You may need to use docker login to create the `~/.docker/config.json` for auth.
#
# docker login

$ sudo buildah push --authfile ~/.docker/config.json rust-example-hello docker://docker.io/secondstate/rust-example-hello:crun
```
