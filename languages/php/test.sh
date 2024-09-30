#!/bin/sh
cargo build --package bitwarden-c
composer install
./vendor/bin/phpunit tests/*Tests.php
