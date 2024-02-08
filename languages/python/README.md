# Build locally
## Requirements

- Python 3
- `maturin` (install with `pip install maturin`)
- `npm`

## Build

From the root of the repository:
```bash
npm run schemas # generate schemas.py

cd languages/python/
python3 -m venv .venv
maturin develop
```

You can now import `BitwardenClient` in your Python code with:
```python
from bitwarden_sdk import BitwardenClient
```

# Use without building locally

```bash
pip install bitwarden-sdk
```

# Run

Set the `ORGANIZATION_ID` and `ACCESS_TOKEN` environment variables to your organization ID and access token, respectively.

```bash
python3 ./example.py
```

## Using Virtual Environments

If you would like to run the script within a virtual environment you can do the following.

```bash
source .venv/bin/activate
python3 ./example.py

deactivate # run this to close the virtual session
```
