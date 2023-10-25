# Run examples
## Build library
```sh
cargo build --release # rust must be installed
```
## Compile example
```sh
gcc -lm examples/distributed.c ./target/release/libdistributed_random_cxx.a -o example
```
## Run example
```sh
./example
```
