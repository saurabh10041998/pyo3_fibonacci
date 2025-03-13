import time

from pyo3_fibonacci import fibonacci as rust_fibonacci

def python_fibonacci(n: int) -> int:
    """Pure Python implementation of Fibonacci"""
    if n <= 1:
        return n
    
    a, b = 0, 1
    for _ in range(1, n):
        a, b = b, a + b
    
    return b


def main():
    start_time = time.monotonic()
    rust_result = rust_fibonacci(40)
    rust_time = time.monotonic() - start_time
    print(f'Rust fibonacci(40) = {rust_result}, took {rust_time:.6f} seconds')

    start_time = time.monotonic()
    python_result = python_fibonacci(40)
    python_time = time.monotonic() - start_time
    print(f'Python fibonacci(40) = {python_result}, took {python_time:.6f} seconds')

    speedup = python_time / rust_time
    print(f'Rust is {speedup:.2f}x faster than Python for this calculation')
    return 0


if __name__ == "__main__":
    raise SystemExit(main())