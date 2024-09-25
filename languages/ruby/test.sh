#!/bin/sh
cargo build --package bitwarden-c
cd spec
bundle install
bundle exec rspec e2e_spec.rb
