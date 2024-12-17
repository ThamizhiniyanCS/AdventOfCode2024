import time
from functools import cache

# My Initial Solution
# def blink(stones):
#     temp = []

#     for stone in stones:
#         if stone == "0":
#             temp.append("1")

#         elif len(stone) % 2 == 0:
#             divide_len = len(stone) // 2

#             temp.append(str(int(stone[:divide_len])))
#             temp.append(str(int(stone[divide_len:])))
#         else:
#             temp.append(str(int(stone) * 2024))

#     return temp


# https://www.youtube.com/watch?v=JxIAqiUJraE
@cache
def count_stones(stone: str, blinks: int) -> int:
    if blinks == 0:
        return 1

    if stone == "0":
        return count_stones("1", blinks - 1)

    len_stone = len(stone)
    if len_stone % 2 == 0:
        divide_len = len(stone) // 2
        return count_stones(str(int(stone[:divide_len])), blinks - 1) + count_stones(
            str(int(stone[divide_len:])), blinks - 1
        )

    return count_stones(str(int(stone) * 2024), blinks - 1)


def part_1(initial_arrangement):
    stones = initial_arrangement.split()

    sum = 0

    for stone in stones:
        sum += count_stones(stone, 25)

    print("[+] Part One Solution")
    print(f">>> Number of stones after 25 blinks: {sum}")


def part_2(initial_arrangement):
    stones = initial_arrangement.split()

    sum = 0

    for stone in stones:
        sum += count_stones(stone, 75)

    print("[+] Part Two Solution")
    print(f">>> Number of tones after 75 blinks: {sum}")


def main():
    with open("../inputs/Day 11: Plutonian Pebbles/input.txt") as file:
        initial_arrangement = file.read().strip()

        part_1(initial_arrangement)
        print()
        part_2(initial_arrangement)


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
