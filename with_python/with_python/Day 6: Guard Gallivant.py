import time

DIRECTION = "u"
GUARD_POSITION = list()
GUARD_POSITIONS = set()
OBSTACLES = set()
MAP = list()
INITIAL_GUARD_POSITION = list()
NO_OF_ROWS = 0
NO_OF_COLUMNS = 0


def up():
    global DIRECTION, GUARD_POSITION, OBSTACLES

    if (GUARD_POSITION[0] - 1, GUARD_POSITION[1]) not in OBSTACLES:
        GUARD_POSITION = [GUARD_POSITION[0] - 1, GUARD_POSITION[1]]
    else:
        DIRECTION = "r"


def right():
    global DIRECTION, GUARD_POSITION, OBSTACLES

    if (GUARD_POSITION[0], GUARD_POSITION[1] + 1) not in OBSTACLES:
        GUARD_POSITION = [GUARD_POSITION[0], GUARD_POSITION[1] + 1]
    else:
        DIRECTION = "d"


def down():
    global DIRECTION, GUARD_POSITION, OBSTACLES

    if (GUARD_POSITION[0] + 1, GUARD_POSITION[1]) not in OBSTACLES:
        GUARD_POSITION = [GUARD_POSITION[0] + 1, GUARD_POSITION[1]]
    else:
        DIRECTION = "l"


def left():
    global DIRECTION, GUARD_POSITION, OBSTACLES

    if (GUARD_POSITION[0], GUARD_POSITION[1] - 1) not in OBSTACLES:
        GUARD_POSITION = [GUARD_POSITION[0], GUARD_POSITION[1] - 1]
    else:
        DIRECTION = "u"


def part_1():
    global DIRECTION, INITIAL_GUARD_POSITION, GUARD_POSITION, GUARD_POSITIONS, NO_OF_ROWS, NO_OF_COLUMNS

    GUARD_POSITION = INITIAL_GUARD_POSITION

    while (
        0 <= GUARD_POSITION[0] < NO_OF_ROWS and 0 <= GUARD_POSITION[1] < NO_OF_COLUMNS
    ):
        GUARD_POSITIONS.add((GUARD_POSITION[0], GUARD_POSITION[1]))

        if DIRECTION == "u":
            up()
        elif DIRECTION == "r":
            right()
        elif DIRECTION == "d":
            down()
        elif DIRECTION == "l":
            left()

    print("[+] Part One Solution")
    print(f">>> Sum: {len(GUARD_POSITIONS)}")


def part_2():
    global DIRECTION, INITIAL_GUARD_POSITION, GUARD_POSITION, GUARD_POSITIONS, NO_OF_ROWS, NO_OF_COLUMNS

    possible_positions_count = 0

    for i in range(NO_OF_ROWS):
        for j in range(NO_OF_COLUMNS):

            if (i, j) in OBSTACLES:
                continue

            DIRECTION = "u"
            GUARD_POSITION = INITIAL_GUARD_POSITION
            GUARD_POSITIONS = set()

            OBSTACLES.add((i, j))

            while (
                0 <= GUARD_POSITION[0] < NO_OF_ROWS
                and 0 <= GUARD_POSITION[1] < NO_OF_COLUMNS
            ):
                if (GUARD_POSITION[0], GUARD_POSITION[1], DIRECTION) in GUARD_POSITIONS:
                    possible_positions_count += 1
                    break

                GUARD_POSITIONS.add((GUARD_POSITION[0], GUARD_POSITION[1], DIRECTION))

                if DIRECTION == "u":
                    up()
                elif DIRECTION == "r":
                    right()
                elif DIRECTION == "d":
                    down()
                elif DIRECTION == "l":
                    left()

            OBSTACLES.remove((i, j))

    print("[+] Part Two Solution")
    print(f">>> Possible Positions Count: {possible_positions_count}")


def main():
    with open("../inputs/Day 6: Guard Gallivant/input.txt") as file:
        global MAP, NO_OF_COLUMNS, NO_OF_ROWS, OBSTACLES, INITIAL_GUARD_POSITION, GUARD_POSITION

        MAP = file.read().splitlines()
        NO_OF_ROWS = len(MAP)
        NO_OF_COLUMNS = len(MAP[0])

        for i, line in enumerate(MAP):
            for j, character in enumerate(line):
                if character == "#":
                    OBSTACLES.add((i, j))
                if character == "^":
                    INITIAL_GUARD_POSITION = [i, j]

        part_1()
        print()
        part_2()


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
