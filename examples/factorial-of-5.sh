#!/bin/bash

set -e

cargo run -- examples/factorial.txt --registers "[(1,5)]"
