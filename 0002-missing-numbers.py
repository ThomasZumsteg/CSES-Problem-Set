if __name__ == "__main__":
    num = int(input())
    nums = set(int(n) for n in input().split())
    print(' '.join(str(n) for n in range(1, num+1) if n not in nums))
