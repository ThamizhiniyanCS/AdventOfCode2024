import time
from collections import defaultdict
from itertools import combinations


def is_valid(elements: list, min: int, max: int):
    for each in elements:
        if not min <= each < max:
            return False

    return True


def part_1(no_of_rows: int, no_of_columns: int, antenna_locations: dict):
    antinode_locations = set()

    for locations in antenna_locations.values():
        for pair in list(combinations(locations, 2)):
            pair_0_x = pair[0][0]
            pair_0_y = pair[0][1]
            pair_1_x = pair[1][0]
            pair_1_y = pair[1][1]

            x_diff = pair_1_x - pair_0_x
            y_diff = pair_1_y - pair_0_y

            pair_0_antinode = (pair_0_x - x_diff, pair_0_y - y_diff)
            pair_1_antinode = (pair_1_x + x_diff, pair_1_y + y_diff)

            if is_valid(pair_0_antinode, 0, no_of_rows):
                antinode_locations.add(pair_0_antinode)
            if is_valid(pair_1_antinode, 0, no_of_columns):
                antinode_locations.add(pair_1_antinode)

    print("[+] Part One Solution")
    print(f">>> Total Anitnodes: {len(antinode_locations)}")


def part_2(no_of_rows: int, no_of_columns: int, antenna_locations: dict):
    antinode_locations = set()

    for locations in antenna_locations.values():
        for pair in list(combinations(locations, 2)):
            antinode_locations.add(tuple(pair[0]))
            antinode_locations.add(tuple(pair[1]))

            pair_0_x = pair[0][0]
            pair_0_y = pair[0][1]
            pair_1_x = pair[1][0]
            pair_1_y = pair[1][1]

            x_diff = pair_1_x - pair_0_x
            y_diff = pair_1_y - pair_0_y

            pair_0_break = False
            pair_0_antinode = (pair_0_x - x_diff, pair_0_y - y_diff)

            if is_valid(pair_0_antinode, 0, no_of_rows):
                antinode_locations.add(pair_0_antinode)

            while pair_0_break == False:
                pair_0_antinode = (
                    pair_0_antinode[0] - x_diff,
                    pair_0_antinode[1] - y_diff,
                )

                if is_valid(pair_0_antinode, 0, no_of_rows):
                    antinode_locations.add(pair_0_antinode)
                else:
                    pair_0_break = True

            pair_1_break = False
            pair_1_antinode = (pair_1_x + x_diff, pair_1_y + y_diff)

            if is_valid(pair_1_antinode, 0, no_of_columns):
                antinode_locations.add(pair_1_antinode)

            while pair_1_break == False:
                pair_1_antinode = (
                    pair_1_antinode[0] + x_diff,
                    pair_1_antinode[1] + y_diff,
                )

                if is_valid(pair_1_antinode, 0, no_of_columns):
                    antinode_locations.add(pair_1_antinode)
                else:
                    pair_1_break = True

    print("[+] Part Two Solution")
    print(f">>> Total Anitnodes: {len(antinode_locations)}")


def main():
    with open("../inputs/Day 8: Resonant Collinearity/input.txt") as file:
        MAP = []
        antenna_locations = defaultdict(list)

        for i, line in enumerate(file.read().splitlines()):
            characters = []
            for j, character in enumerate(line):
                characters.append(character)
                if not character == ".":
                    antenna_locations[character].append([i, j])

            MAP.append(characters)

        no_of_rows = len(MAP)
        no_of_columns = len(MAP[0])

        part_1(no_of_rows, no_of_columns, antenna_locations)
        print()
        part_2(no_of_rows, no_of_columns, antenna_locations)


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
