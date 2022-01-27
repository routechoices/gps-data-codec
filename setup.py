import sys

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="gps-encoding-rust",
    version="1.2.0",
    rust_extensions=[RustExtension("gps_encoding.gps_encoding", binding=Binding.RustCPython)],
    packages=["gps_encoding"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    long_description="Base funtions for gps data encoding implemented in rust.",
    long_description_content_type="text/x-rst"
)