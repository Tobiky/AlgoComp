import sys
from typing import Union, List, Dict

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
        values = list(map(lambda x: 1 + a(required - x, new_lookup), new_lookup))
        values.append(required)
        return min(values)

b = a

def c(required: Union[int, float], lookup: List[int], memory: Union[Dict[int, int], None] = None):
    if required < 0:
        return float("inf")
    elif required == 0:
        return 0
    else:
        if memory == None:
            memory = dict() 

        if required in memory:
            return memory[required]
        else:
            new_lookup = lookup # list(filter(lambda x: x <= required, lookup))

            def calc(x):
                return 1 + c(required - x, new_lookup, memory)

            values = list(map(calc, new_lookup))
            values.append(required)

            minimum = min(values)
            memory[required] = minimum
            return minimum
    pass

def d(required: int, lookup: List[int]):
    pass

def e(required: int, lookup: List[int]):
    pass

def read_input(inp: Union[List[str], None] = None):
    values = list(map(int, inp or sys.stdin.readlines()))
    return values[0], values[1::]

def setup():
    sys.setrecursionlimit(100_000)

def teardown():
    pass

if __name__ == "__main__":
    setup()

    required, lookup = read_input()
    lookup.sort()
    
    __slot__ = ("memory",)
    memory = dict() 

    coins = c(required, lookup, memory)
    print(coins)

    teardown()
