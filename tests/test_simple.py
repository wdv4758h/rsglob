import glob as glob1
import rsglob as glob2


def test_nondot_files():
    assert set(glob1.glob("*")) == set(glob2.glob("*"))

def test_dot_files():
    assert set(glob1.glob(".*")) == set(glob2.glob(".*"))
