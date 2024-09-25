#!/bin/sh
cd spec
bundle install
bundle exec rspec e2e_spec.rb
