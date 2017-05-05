#!/bin/bash

MY_DIR=$(dirname $0)

# export CARGO_INCREMENTAL=1
watchexec -f '*.less' "$MY_DIR/do-less.sh" &
watchexec --restart -i '*.less' -i '*.svg' -i '*.sql' "clr && git status -s && echo '------------' && cargo build && RUST_BACKTRACE=1 ./target/debug/basgit"
