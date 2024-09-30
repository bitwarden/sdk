#!/bin/sh
composer install
./vendor/bin/phpunit tests/*Tests.php
