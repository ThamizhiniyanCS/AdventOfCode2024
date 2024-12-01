import time
from collections import Counter


def part_1(list_1, list_2):
    total_distance = sum(
        [
            abs(value_1 - value_2)
            for value_1, value_2 in zip(sorted(list_1), sorted(list_2))
        ]
    )

    print("[+] Part One Solution")
    print(f">>> Total Distance: {total_distance}")


def part_2(list_1, list_2):
    dictionary = Counter(list_2)

    similarity_score = sum([value * dictionary[value] for value in list_1])

    print("[+] Part Two Solution")
    print(f">>> Similarity Score: {similarity_score}")


def main():
    with open("../inputs/Day 1: Historian Hysteria/input.txt") as file:
        list_1, list_2 = [], []

        for line in file:
            value_1, value_2 = map(int, line.strip().split())

            list_1.append(value_1)
            list_2.append(value_2)

        part_1(list_1, list_2)
        print()
        part_2(list_1, list_2)


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
