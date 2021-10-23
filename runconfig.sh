#!/usr/bin/env bash
eval $(grep -v '^#' sample.env | xargs) cargo run --bin config
