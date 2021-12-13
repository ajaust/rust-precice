#/usr/bin/env sh

# ensure that bindgen is installed
# cargo install bindgen

bindgen wrapper.h -o src/bindings.rs