import pytest
import pyo3_fibonacci


def test_sum_as_string():
    assert pyo3_fibonacci.sum_as_string(1, 1) == "2"
