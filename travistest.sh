#!/bin/bash

nohup q heartbeat.q -p 5000 < /dev/null >> qtest.log 2>&1 &
cargo run --example test --release
