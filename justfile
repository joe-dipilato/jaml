
project_name := "jcode"

# help
help:
    just --list
# Pull rust image
_pull_ctr:
    docker pull rust
# Exec rust image
build_ctr: _pull_ctr
    docker build --file Containerfile -t rust_osx .
# Exec rust image
exec_ctr:
    docker run -it -v ${PWD}:/workspace -w /workspace rust_osx
build2:
    docker run -it -v ${PWD}:/workspace -w /workspace rust_osx cargo build --target x86_64-apple-darwin hello.rs
build: (_ctr_cmd "build --manifest-path " + project_name + "/Cargo.toml --target-dir build")
# Clean the build directory
@clean:
    mkdir -p build
    rm -rf build
    mkdir build
# container command
_ctr_cmd cmd="":
    docker run -it -v ${PWD}:/workspace -w /workspace rust_osx cargo {{cmd}}
# One time init
__init_cargo: (_ctr_cmd "new " + project_name)
