from setuptools import setup
from setuptools_rust import RustExtension


setup(
    name="emr-udfs",
    version="0.0.1",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
    ],
    packages=["emr_udfs"],
    rust_extensions=[RustExtension("emr_udfs.emr_udfs", "Cargo.toml", debug=False)],
    include_package_data=True,
    zip_safe=False,
)
