import sys
from typing import Union, List

# if digit is None:
#     digit = len(lookup) - 1
# result = required if digit == -1 else required // lookup[digit] + a(required % lookup[digit], lookup, digit - 1)
# return result

def a(required: int, lookup: List[int]):
    if required < 0:
        return float("inf")
    elif required == 0:
        return 0
    else:
        new_lookup = list(filter(lambda x: x <= required, lookup))
        values = list(map(lambda x: a(required - x, new_lookup), new_lookup))
        values.append(required)
        return 1 + min(values)

def b(required: int, lookup: List[int]):
    pass

def c(required: int, lookup: List[int]):
    pass

def d(required: int, lookup: List[int]):
    pass

def e(required: int, lookup: List[int]):
    pass

def read_input(inp: Union[List[str], None] = None):
    values = list(map(int, inp or sys.stdin.readlines()))
    return values[0], values[1::]

def setup():
    sys.setrecursionlimit(5000)

def teardown():
    pass

if __name__ == "__main__":
    setup()

    required, lookup = read_input()
    lookup.sort()
    coins = a(required, lookup)
    print(coins)

    teardown()
