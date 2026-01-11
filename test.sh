#!/bin/bash

set -e

cargo build

RUST_BACKTRACE=FULL ./target/debug/interpreter tokenize ./reef/hello.reef
