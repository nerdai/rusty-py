# RustyPyCore

## Dependency management

```
poetry install
poetry add <>
```

## Building Rust binaries and adding them to Python package

```
maturin develop
```

## Publishing to PyPi

```
export MATURIN_PYPI_TOKEN=<>
maturin build
maturin publish
```

## Installation

```
pip install rusty-py-core
```

## Usage

```python
from rusty_py.core import add

result = add(left=2, right=2)
```