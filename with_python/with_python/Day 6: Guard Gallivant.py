import time
from collections import defaultdict


def part_1(
    map,
    no_of_rows: int,
    no_of_columns: int,
    obstacles: dict,
    initial_guard_position,
):
    direction = "u"
    guard_position = initial_guard_position
    no_obstacle = True
    guard_positions = set()
    guard_positions.add(f"{initial_guard_position[0]}_{initial_guard_position[1]}")
    exit_position = []
    is_loop = False
    map_guard_positions = [initial_guard_position]

    counter = 0

    while no_obstacle and counter < 50:
        print(f"Counter: {counter}")
        print(f"Direction: {direction}")
        print(f"Guard Position: {guard_position}")

        if direction == "u":
            if guard_position[0] == 0:
                exit_position = guard_position
                no_obstacle = False
                break

            for i in range(guard_position[0], 0, -1):
                key = f"{i - 1}_{guard_position[1]}"
                print(key)
                if [i - 1, guard_position[1]] == initial_guard_position:
                    is_loop = True
                    no_obstacle = False
                    break

                if obstacles.get(key):
                    direction = "r"
                    guard_position = [i, guard_position[1]]
                    guard_positions.add(f"{i}_{guard_position[1]}")
                    map_guard_positions.append(guard_position)
                    break

                elif key not in guard_positions and i > 0:
                    guard_position = [i - 1, guard_position[1]]
                    guard_positions.add(key)
                    map_guard_positions.append(guard_position)

        elif direction == "r":
            if guard_position[1] + 1 == no_of_columns:
                exit_position = guard_position
                no_obstacle = False
                break

            for i in range(guard_position[1] + 1, no_of_columns):
                key = f"{guard_position[0]}_{i}"
                print(key)
                if [guard_position[0], i] == initial_guard_position:
                    is_loop = True
                    no_obstacle = False
                    break

                if obstacles.get(key):
                    direction = "d"
                    guard_position = [guard_position[0], i - 1]
                    guard_positions.add(f"{guard_position[0]}_{i - 1}")
                    map_guard_positions.append(guard_position)
                    break

                elif key not in guard_positions and i < no_of_columns:
                    guard_position = [guard_position[0], i]
                    guard_positions.add(key)
                    map_guard_positions.append(guard_position)

        elif direction == "d":
            if guard_position[0] + 1 == no_of_rows:
                exit_position = guard_position
                no_obstacle = False
                break

            for i in range(guard_position[0] + 1, no_of_rows):
                key = f"{i}_{guard_position[1]}"
                print(key)
                if [i, guard_position[1]] == initial_guard_position:
                    is_loop = True
                    no_obstacle = False
                    break

                if obstacles.get(key):
                    direction = "l"
                    guard_position = [i - 1, guard_position[1]]
                    guard_positions.add(f"{i - 1}_{guard_position[1]}")
                    map_guard_positions.append(guard_position)
                    break

                elif key not in guard_positions and i < no_of_rows:
                    guard_position = [i, guard_position[1]]
                    guard_positions.add(key)
                    map_guard_positions.append(guard_position)

        elif direction == "l":
            if guard_position[1] == 0:
                exit_position = guard_position
                no_obstacle = False
                break

            for i in range(guard_position[1], 0, -1):
                key = f"{guard_position[0]}_{i - 1}"
                print(key)
                if [guard_position[0], i - 1] == initial_guard_position:
                    is_loop = True
                    no_obstacle = False
                    break

                if obstacles.get(key):
                    direction = "u"
                    guard_position = [guard_position[0], i]
                    guard_positions.add(f"{guard_position[0]}_{i}")
                    map_guard_positions.append(guard_position)
                    break

                elif key not in guard_positions and i > 0:
                    guard_position = [guard_position[0], i - 1]
                    guard_positions.add(key)
                    map_guard_positions.append(guard_position)

        counter += 1

    print(f"map_guard_positions: {map_guard_positions}")

    for each in map_guard_positions:
        map[each[0]][each[1]] = "X"

    for line in map:
        print("".join(line))

    print(f"Guard Positions: {sorted(guard_positions)}")

    print("[+] Part One Solution")
    print(f">>> Sum: {len(guard_positions)}")
    print(f"Exit Position: {exit_position}")

    return is_loop


def part_2(
    map, no_of_rows: int, no_of_columns: int, obstacles: dict, initial_guard_position
):
    # map[6][3] = "#"
    i, j = 9, 7
    obstacles[f"{i}_{j}"] = [i, j]

    print(
        part_1(
            map,
            no_of_rows,
            no_of_columns,
            obstacles,
            initial_guard_position,
        )
    )

    # print("[+] Part Two Solution")
    # print(f">>> Sum of middle values of valid updates: ")


def main():
    with open("../inputs/Day 6: Guard Gallivant/sample-input.txt") as file:
        map = []
        obstacles = {}
        initial_guard_position = []

        for i, line in enumerate(file.read().splitlines()):
            characters = []
            for j, character in enumerate(line):
                characters.append(character)
                if character == "#":
                    key = f"{i}_{j}"
                    if not obstacles.get(key):
                        obstacles[key] = [i, j]
                    else:
                        print(f"Duplicate Obstacle -> {key}: [{i}, {j}]")
                if character == "^":
                    initial_guard_position = [i, j]

            map.append(characters)

        no_of_rows = len(map)
        no_of_columns = len(map[0])

        # print(obstacles)

        part_1(
            map,
            no_of_rows,
            no_of_columns,
            obstacles,
            initial_guard_position,
        )
        print()
        # part_2(
        #     map,
        #     no_of_rows,
        #     no_of_columns,
        #     obstacles,
        #     initial_guard_position,
        # )


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
