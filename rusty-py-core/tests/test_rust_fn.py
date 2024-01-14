from rusty_py.core import add

def test_rust_add():
    result = add(2,3)
    assert result == 5