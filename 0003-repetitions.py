def longest(chars):
    last = None
    current = 0
    longest = 0
    for char in chars:
        if last != char:
            current = 0
            last = char
        current += 1
        longest = max(current, longest)
    return longest

if __name__ == "__main__":
    print(longest(input()))

def test_simple():
    assert longest("") == 0
    assert longest("A") == 1
    assert longest("AA") == 2
    assert longest("AAGG") == 2
    assert longest("AAGGTTT") == 2
