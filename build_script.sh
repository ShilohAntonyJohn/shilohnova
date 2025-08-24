#!/bin/bash

# This script sets the necessary environment variables for building with clang and then runs the Leptos watcher.

# Set the C++ compiler to clang++.
export CXX=clang++

# Tell the C++ compiler to automatically include the cstdint header, which is
# needed by the RocksDB library to define integer types like uint64_t.
export CXXFLAGS="-include cstdint"

# Run the Leptos watcher. It will automatically use the CXX and CXXFLAGS
# variables we've just set.
cargo leptos watch
