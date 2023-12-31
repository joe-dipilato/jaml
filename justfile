
project_name := "jcode"
default_container := "rust_basic"

# help
help:
    just --list
# Exec rust image
build_osx_ctr:
    docker build --file containers/rust_osx/Containerfile -t rust_osx containers/rust_osx/
build_ctr:
    docker build --file containers/rust_basic/Containerfile -t rust_basic .
# Exec rust image
exec_ctr: (_ctr_cmd)
build: (_ctr_cmd_full "build")
release: (_ctr_cmd_full "build --release")
check: (_ctr_cmd_full "check")
fix: (_ctr_cmd_full "fix --allow-dirty") (_ctr_cmd_full "clippy")
run args="": (_ctr_cmd_full "run" "-- " + args)
# container command
_ctr_cmd cmd="":
    time docker run -it -v ${PWD}:/workspace -w /workspace {{default_container}} {{cmd}}
# container command
_ctr_cmd_full cmd="" args="": (_ctr_cmd "cargo " + cmd + " --manifest-path " + project_name + "/Cargo.toml " + args )
# One time init
__init_cargo: (_ctr_cmd "cargo new " + project_name)
# Clean the build directory
@clean:
    mkdir -p build
    rm -rf build
    mkdir build
