# Requirements

- Python3
- setuptools
  ```bash
  pip install setuptools
  ```
- setuptools_rust
  ```bash
  pip install setuptools_rust
  ```
- dateutil
  ```bash
  pip install python-dateutil
  ```

# Installation

From the `languages/python/` directory,

```bash
python3 ./setup.py develop
```

Rename the the resulting `.so` file to `bitwarden_py.so`, if it isn't already there.

# Run

```bash
python3 ./login.py
```
