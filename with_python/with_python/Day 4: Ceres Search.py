import time


def get_positions(length, i, j):
    i_minus_1 = i - 1 if i - 1 >= 0 else None
    i_plus_1 = i + 1 if i + 1 < length else None
    j_minus_1 = j - 1 if j - 1 >= 0 else None
    j_plus_1 = j + 1 if j + 1 < length else None

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


def find_X_positions(length, content):
    x_positions = []

    for i in range(len(content)):
        for j in range(length):
            if content[i][j] == "X":
                x_positions.append([i, j])

    return x_positions


def find_M_positions(length, content, i, j):
    positions = get_positions(length, i, j)

    m_positions = {}

    for key, value in positions.items():
        if value and content[value[0]][value[1]] == "M":
            m_positions[key] = value

    return m_positions


def check_A(length, content, direction, i, j):
    positions = get_positions(length, i, j)

    if (
        positions[direction]
        and content[positions[direction][0]][positions[direction][1]] == "A"
    ):
        return [positions[direction][0], positions[direction][1]]
    else:
        return False


def check_S(length, content, direction, i, j):
    positions = get_positions(length, i, j)

    if (
        positions[direction]
        and content[positions[direction][0]][positions[direction][1]] == "S"
    ):
        return [positions[direction][0], positions[direction][1]]
    else:
        return False


def part_1(content):
    total_times = 0
    length = len(content[0])
    counter = 0

    dotted_map = [["."] * length for _ in range(len(content))]

    for x in find_X_positions(length, content):
        for m_key, m_value in find_M_positions(length, content, x[0], x[1]).items():
            result_a = check_A(length, content, m_key, m_value[0], m_value[1])
            if result_a:
                result_s = check_S(length, content, m_key, result_a[0], result_a[1])

                if result_s:

                    x_position = [x[0], x[1]]
                    m_position = [m_value[0], m_value[1]]
                    a_position = [result_a[0], result_a[1]]
                    s_position = [result_s[0], result_s[1]]

                    dotted_map[x_position[0]][x_position[1]] = content[x_position[0]][
                        x_position[1]
                    ]
                    dotted_map[m_position[0]][m_position[1]] = content[m_position[0]][
                        m_position[1]
                    ]
                    dotted_map[a_position[0]][a_position[1]] = content[a_position[0]][
                        a_position[1]
                    ]
                    dotted_map[s_position[0]][s_position[1]] = content[s_position[0]][
                        s_position[1]
                    ]

                    counter += 4

                    total_times += 1

    print("[+] Part One Solution")
    print(f">>> Total Times XMAS Appeared: {total_times}")


def part_2(content):

    print("[+] Part Two Solution")
    print(f">>> ")


def main():
    with open("../inputs/Day 4: Ceres Search/input.txt") as file:
        content = [line.strip() for line in file.readlines()]

        print()
        part_1(content)


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
