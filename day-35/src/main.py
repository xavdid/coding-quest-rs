from functools import cache

# STEPS = [1, 2, 3]
# INPUT = 5
# ---
STEPS = [40, 12, 2, 1]
INPUT = 856


@cache
def ways_to_make(i: int) -> int:
    if i < 0:
        return 0
    if i == 0:
        return 1

    return sum(ways_to_make(i - step) for step in STEPS)


if __name__ == "__main__":
    print(ways_to_make(INPUT))
