def spiral(x, y):
    # 1 + 2 + 4 + 6
    # 1 + 2 * (1 + 2 + 3 + 4 ..)
    assert x > 0 and y > 0
    n = max(x, y) - 1
    middle_num = 1 + n * (n + 1)
    if (x > y and x % 2 == 0) or (x < y and y % 2 == 0):
        return middle_num + x - y
    else:
        return middle_num + y - x



if __name__ == "__main__":
    count = int(input())
    nums = []
    for _ in range(count):
        x, y = input().split(' ')
        print(spiral(int(x), int(y)))


def test_simple():
    assert spiral(1, 1) == 1
    assert spiral(2, 2) == 3
    assert spiral(3, 3) == 7
    assert spiral(4, 4) == 13
    assert spiral(1, 2) == 2
    assert spiral(2, 2) == 3
    assert spiral(2, 1) == 4
    assert spiral(3, 1) == 5
    assert spiral(3, 2) == 6
    assert spiral(3, 3) == 7
    assert spiral(2, 3) == 8
    assert spiral(1, 3) == 9
    assert spiral(1, 4) == 10
    assert spiral(4, 2) == 15
