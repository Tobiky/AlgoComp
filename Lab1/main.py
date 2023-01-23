#!/usr/bin/python
from importlib import import_module
from time import time_ns
from typing import Any, Callable
from multiprocessing import Process

max_timeout = 3 * 60 # 3 min

def time_process(func: Callable[..., Any], args: list, timeout: int | None = None, repitions: int | None = None):
    total_time = 0
    repitions = repitions or 1

    for _ in range(repitions):
        process = Process(target=func, args=args)

        process.start()
        start = time_ns()

        process.join(timeout or max_timeout) # seconds
        if process.is_alive():
            process.terminate()

        stop = time_ns()

        duration = stop - start

        total_time += duration

    return total_time / repitions

def doubled(n: int):
    state = n
    while True:
        yield state
        state = state * 2

def linear(n: int):
    state = n
    while True:
        yield state
        state += 1

def constant(n: int):
    return [n]

if __name__ == "__main__":
    ns_time_const = 10 ** 9

    func = "b"
    mod  = "coin"
    module = import_module(mod)

    coins = [5, 6, 7]

    get_duration = lambda size: time_process(
            func=getattr(module, func),
            args=[size, coins])

    calculations: list[tuple[float, int]] = []

    for size in linear(1):
        duration = get_duration(size)
        print(f"# Attempting: ({size}, {duration / ns_time_const})")

        calculations.append((duration, size))

        if duration / ns_time_const > 1.5: # larger than 3 min
            break

    closest_n = sorted(calculations, key=lambda x: abs(x[0] - ns_time_const))[0]

    print(f"# Found closest N to be {closest_n[1]}, at {closest_n[0] / ns_time_const} s")

    print("# Linear Increase")
    print("# size, s")
    for size in linear(closest_n[1]):
        duration = get_duration(size)

        print(f"{size},{duration / ns_time_const}")

        if duration / ns_time_const > max_timeout: # larger than 3 min
            break

    print("# Doubling Increase")
    print("# size, s")
    for size in doubled(closest_n[1]):
        duration = get_duration(size)

        print(f"{size},{duration / ns_time_const}")

        if duration / ns_time_const > max_timeout: # larger than 3 min
            break

