import itertools
import typing

def increasing(nums: typing.Iterable[int]) -> int:
    nums = iter(nums)
    try:
        last = next(nums)
    except StopIteration:
        return 0
    moves = 0
    for num in nums:
        if num < last:
            moves += last - num
        else:
            last = num
    return moves 

if __name__ == "__main__":
    count = int(input())
    nums = (int(n) for n in itertools.islice(input().split(' '), count))
    print(increasing(nums))

def test_simple():
    assert increasing([]) == 0
    assert increasing([0]) == 0
    assert increasing([0, 1]) == 0
    assert increasing([1, 0]) == 1
    assert increasing([1, 0, 2]) == 1
    assert increasing([2, 0, 1]) == 3
    assert increasing([3, 2, 5, 1, 7]) == 5
