from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="BitwardenClient",
    description="A Bitwarden Client for python",
    version="0.1",
    rust_extensions=[RustExtension(
        "bitwarden_py", path="../../crates/bitwarden-py/Cargo.toml", binding=Binding.PyO3)],
    packages=['bitwardenclient'],
    zip_safe=False,
)
