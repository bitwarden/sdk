# Build locally
## Requirements

- Python 3
- `maturin` (install with `pip install maturin[patchelf]`)
- `npm`

## Build

From the root of the repository:
```bash
npm run schemas # generate schemas.py

cd languages/python/
maturin develop
```

You can now import `BitwardenClient` and `bitwarden_py` in your Python code.

# Use without building locally

```bash
pip install BitwardenClient
```

# Run

Set the `BWS_ORG_ID` and `BWS_ACCESS_TOKEN` environment variables to your organization ID and access token, respectively.

```bash
python3 ./example.py
```
