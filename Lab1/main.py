#!/usr/bin/python
from importlib import import_module
from multiprocessing import Process
from time import time_ns
from typing import Callable
from threading import Thread

max_timeout = 3 * 60 * 60

def time_process(process_creation: Callable[[], Thread], timeout: int | None = None, repitions: int | None = None):
    total_time = 0
    repitions = repitions or 100

    for _ in range(repitions):
        process = process_creation()

        start = time_ns()
        process.start()

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

    func = "a"
    mod  = "coin"
    module = import_module(mod)

    coins = [5, 6, 7]

    closest_n = (float('inf'), 0)

    get_duration = lambda: time_process(process_creation=lambda: Thread(target=getattr(module, func), args=[size, coins]))

    for size in linear(1):
        duration = get_duration()

        if abs(duration - ns_time_const) < abs(closest_n[0] - ns_time_const):
            closest_n = (duration, size)
        else:
            break

    print("# Linear Increase")
    print("# size, ns")
    for size in linear(closest_n[1]):
        duration = get_duration() 

        print(f"{size},{duration}")

        if duration >= max_timeout:
            break

    print("# Doubling Increase")
    print("# size, ns")
    for size in doubled(closest_n[1]):
        duration = get_duration() 

        print(f"{size},{duration}")

        if duration >= max_timeout:
            break