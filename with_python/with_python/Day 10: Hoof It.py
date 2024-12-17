import time


def get_next_positions(no_of_rows, no_of_columns, i, j):
    return [
        None if i - 1 < 0 else [i - 1, j],
        [i + 1, j] if i + 1 < no_of_rows else None,
        None if j - 1 < 0 else [i, j - 1],
        [i, j + 1] if j + 1 < no_of_columns else None,
    ]


def get_valid_positions(MAP, positions, value):
    valid_positions = []

    for position in positions:
        if position:
            if int(MAP[position[0]][position[1]]) == value:
                valid_positions.append((position[0], position[1]))

    return valid_positions


def get_trailhead_rating_and_score(
    MAP,
    i,
    j,
    no_of_rows,
    no_of_columns,
):
    positions = get_valid_positions(
        MAP, get_next_positions(no_of_rows, no_of_columns, i, j), 1
    )

    for value in range(2, 10):
        temp_positions = []

        for position in positions:
            for valid_position in get_valid_positions(
                MAP,
                get_next_positions(no_of_rows, no_of_columns, position[0], position[1]),
                value,
            ):
                temp_positions.append(valid_position)

        positions = temp_positions

    return (len(positions), len(set(positions)))


def main():
    with open("../inputs/Day 10: Hoof It/input.txt") as file:
        MAP = [line.strip() for line in file.readlines()]
        no_of_rows = len(MAP)
        no_of_columns = len(MAP[0])
        trailheads = []

        for i, line in enumerate(MAP):
            for j, character in enumerate(line):
                if character == "0":
                    trailheads.append((i, j))

        score, rating = 0, 0

        for i, j in trailheads:
            (trailhead_rating, trailhead_score) = get_trailhead_rating_and_score(
                MAP,
                i,
                j,
                no_of_rows,
                no_of_columns,
            )
            score += trailhead_score
            rating += trailhead_rating

        print("[+] Part One Solution")
        print(f">>> Sum of Scores of all Trailheads: {score}")
        print()
        print("[+] Part Two Solution")
        print(f">>> Sum of Rating of all Trailheads: {rating}")


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
