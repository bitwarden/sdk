from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="bitwarden_sdk",
    description="A Bitwarden Client for python",
    version="0.1",
    rust_extensions=[RustExtension(
        "bitwarden_py", path="../../crates/bitwarden-py/Cargo.toml", binding=Binding.PyO3)],
    packages=['bitwarden_sdk'],
    zip_safe=False,
)
