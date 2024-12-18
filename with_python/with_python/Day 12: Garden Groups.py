import time


def get_next_positions(no_of_rows, no_of_columns, i, j):
    return [
        None if i - 1 < 0 else (i - 1, j),
        (i + 1, j) if i + 1 < no_of_rows else None,
        None if j - 1 < 0 else (i, j - 1),
        (i, j + 1) if j + 1 < no_of_columns else None,
    ]


def get_valid_positions(plot, positions, value):
    valid_positions = []

    for position in positions:
        if position:
            x, y = position

            if plot[x][y] == value:
                valid_positions.append((x, y))

    return valid_positions


def enumerate_plant_region(
    plot: list[str],
    no_of_rows: int,
    no_of_columns: int,
    plant: str,
    to_enumerate: tuple[int, int],
    enumerated_positions: set = None,
) -> list:
    if enumerated_positions is None:
        enumerated_positions = set()

    if to_enumerate in enumerated_positions:
        return

    enumerated_positions.add(to_enumerate)
    x, y = to_enumerate

    for position in get_valid_positions(
        plot, get_next_positions(no_of_rows, no_of_columns, x, y), plant
    ):
        enumerate_plant_region(
            plot,
            no_of_rows,
            no_of_columns,
            plant,
            to_enumerate=position,
            enumerated_positions=enumerated_positions,
        )

    return enumerated_positions


def caculate_perimeter(positions: set[tuple[int, int]]):
    sides = list()

    for x, y in positions:
        for position in [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
        ]:

            if position not in positions:
                sides.append(position)

    return len(sides)

# https://www.youtube.com/watch?v=iKCgjy7-2nY
def caculate_sides(region: set[tuple[int, int]]):
    up, down, left, right = (set() for _ in range(4))

    for r, c in region:
        if (r - 1, c) not in region:
            up.add((r, c))
        if (r + 1, c) not in region:
            down.add((r, c))
        if (r, c - 1) not in region:
            left.add((r, c))
        if (r, c + 1) not in region:
            right.add((r, c))

    count = 0
    for r, c in up:
        if (r, c) in left:
            count += 1
        if (r, c) in right:
            count += 1
        if (r - 1, c - 1) in right and (r, c) not in left:
            count += 1
        if (r - 1, c + 1) in left and (r, c) not in right:
            count += 1

    for r, c in down:
        if (r, c) in left:
            count += 1
        if (r, c) in right:
            count += 1
        if (r + 1, c - 1) in right and (r, c) not in left:
            count += 1
        if (r + 1, c + 1) in left and (r, c) not in right:
            count += 1

    return count


def main():
    with open("../inputs/Day 12: Garden Groups/input.txt") as file:
        plot = [line.strip() for line in file.readlines()]
        no_of_rows = len(plot)
        no_of_columns = len(plot[0])
        explored_positions = set()

        total_price_perimeter = 0
        total_price_sides = 0

        for i, line in enumerate(plot):
            for j, character in enumerate(line):
                if (i, j) in explored_positions:
                    continue

                enumerated_positions = enumerate_plant_region(
                    plot, no_of_rows, no_of_columns, character, to_enumerate=(i, j)
                )

                for position in enumerated_positions:
                    explored_positions.add(position)

                total_price_perimeter += len(enumerated_positions) * caculate_perimeter(
                    enumerated_positions
                )
                total_price_sides += len(enumerated_positions) * caculate_sides(
                    enumerated_positions
                )

        print("[+] Part One Solution")
        print(f">>> Price of fencing based on perimeter: {total_price_perimeter}")
        print()
        print("[+] Part Two Solution")
        print(f">>> Price of fencing based on sides: {total_price_sides}")


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
