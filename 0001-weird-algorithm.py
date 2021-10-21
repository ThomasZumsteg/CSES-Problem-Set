def collatz(start: int):
    while True:
        yield start
        if start == 1:
            break
        start = int(start / 2 if start % 2 == 0 else start * 3 + 1)

def test_simple():
    assert list(collatz(1)) == [1]
    assert list(collatz(2)) == [2, 1]
    assert list(collatz(3)) == [3, 10, 5, 16, 8, 4, 2, 1]

if __name__ == "__main__":
    print(" ".join(str(n) for n in collatz(int(input()))))
