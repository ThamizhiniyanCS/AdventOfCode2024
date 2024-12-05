import time


def get_positions(no_of_rows, no_of_columns, i, j):
    i_minus_1 = i - 1 if i - 1 >= 0 else None
    i_plus_1 = i + 1 if i + 1 < no_of_rows else None
    j_minus_1 = j - 1 if j - 1 >= 0 else None
    j_plus_1 = j + 1 if j + 1 < no_of_columns else None

    return {
        "left": [i, j_minus_1] if j_minus_1 is not None else None,
        "right": [i, j_plus_1] if j_plus_1 is not None else None,
        "up": [i_minus_1, j] if i_minus_1 is not None else None,
        "down": [i_plus_1, j] if i_plus_1 is not None else None,
        "top_left": (
            [i_minus_1, j_minus_1]
            if i_minus_1 is not None and j_minus_1 is not None
            else None
        ),
        "top_right": (
            [i_minus_1, j_plus_1]
            if i_minus_1 is not None and j_plus_1 is not None
            else None
        ),
        "bottom_left": (
            [i_plus_1, j_minus_1]
            if i_plus_1 is not None and j_minus_1 is not None
            else None
        ),
        "bottom_right": (
            [i_plus_1, j_plus_1]
            if i_plus_1 is not None and j_plus_1 is not None
            else None
        ),
    }


def find_character_positions(no_of_rows, no_of_columns, content, character):
    return [
        [i, j]
        for j in range(no_of_columns)
        for i in range(no_of_rows)
        if content[i][j] == character
    ]


def find_M_positions(no_of_rows, no_of_columns, content, i, j):
    return {
        key: value
        for key, value in get_positions(no_of_rows, no_of_columns, i, j).items()
        if value and content[value[0]][value[1]] == "M"
    }


def check_character(no_of_rows, no_of_columns, content, direction, i, j, character):
    positions = get_positions(no_of_rows, no_of_columns, i, j)

    if (
        positions[direction]
        and content[positions[direction][0]][positions[direction][1]] == character
    ):
        return [positions[direction][0], positions[direction][1]]
    else:
        return False


def check_x_mas(no_of_rows, no_of_columns, content, i, j) -> bool:
    positions = get_positions(no_of_rows, no_of_columns, i, j)

    position_top_left = positions.get("top_left")
    position_top_right = positions.get("top_right")
    position_bottom_left = positions.get("bottom_left")
    position_bottom_right = positions.get("bottom_right")

    if (
        position_top_left
        and position_top_right
        and position_bottom_left
        and position_bottom_right
    ):
        top_left = content[position_top_left[0]][position_top_left[1]]
        top_right = content[position_top_right[0]][position_top_right[1]]
        bottom_left = content[position_bottom_left[0]][position_bottom_left[1]]
        bottom_right = content[position_bottom_right[0]][position_bottom_right[1]]

        if (
            (
                top_left == "M"
                and top_right == "S"
                and bottom_left == "M"
                and bottom_right == "S"
            )
            or (
                top_left == "M"
                and top_right == "M"
                and bottom_left == "S"
                and bottom_right == "S"
            )
            or (
                top_left == "S"
                and top_right == "S"
                and bottom_left == "M"
                and bottom_right == "M"
            )
            or (
                top_left == "S"
                and top_right == "M"
                and bottom_left == "S"
                and bottom_right == "M"
            )
        ):
            return True

    return False


def part_1(content, no_of_rows, no_of_columns):
    total_times = 0

    for x in find_character_positions(no_of_rows, no_of_columns, content, "X"):
        for m_key, m_value in find_M_positions(
            no_of_rows, no_of_columns, content, x[0], x[1]
        ).items():
            result_a = check_character(
                no_of_rows, no_of_columns, content, m_key, m_value[0], m_value[1], "A"
            )
            if result_a:
                result_s = check_character(
                    no_of_rows,
                    no_of_columns,
                    content,
                    m_key,
                    result_a[0],
                    result_a[1],
                    "S",
                )
                if result_s:
                    total_times += 1

    print("[+] Part One Solution")
    print(f">>> Total Times XMAS Appeared: {total_times}")


def part_2(content, no_of_rows, no_of_columns):
    total_times = 0

    for a in find_character_positions(no_of_rows, no_of_columns, content, "A"):
        if check_x_mas(no_of_rows, no_of_columns, content, a[0], a[1]):
            total_times += 1

    print("[+] Part Two Solution")
    print(f">>> Total Times X-MAS Appeared: {total_times}")


def main():
    with open("../inputs/Day 4: Ceres Search/input.txt") as file:
        content = [line.strip() for line in file.readlines()]
        no_of_rows = len(content)
        no_of_columns = len(content[0])

        print()
        part_1(content, no_of_rows, no_of_columns)
        print()
        part_2(content, no_of_rows, no_of_columns)


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
