#!/bin/bash

export CARGO_INCREMENTAL=1
watchexec -f '*.less' "lessc less/main.less static/main.css && echo '~~~ lessc: done ~~~'" &
watchexec --restart -i '*.less' "clr && git status -s && echo '------------' && cargo build && RUST_BACKTRACE=1 ./target/debug/basgit"
