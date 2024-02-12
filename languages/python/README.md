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
