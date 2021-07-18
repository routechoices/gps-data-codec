import sys

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="polyline_encoding_rust",
    version="1.0",
    rust_extensions=[RustExtension("polyline_encoding.polyline_encoding", binding=Binding.RustCPython)],
    packages=["polyline_encoding"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    long_description="Base funtions for polyline encoding implemented in rust.",
    long_description_content_type="text/x-rst"
)