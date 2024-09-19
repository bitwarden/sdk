#!/bin/sh
python3 -m venv .venv
. .venv/bin/activate
pip3 install maturin
maturin develop
python3 test.py
