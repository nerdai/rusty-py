Poetry for dependency management

```
poetry install
poetry add <>
```

Building Rust binaries and adding them to Python package

```
maturin develop
```

Publishing to PyPi

```
export MATURIN_PYPI_TOKEN=<>
maturin build
maturin publish
```