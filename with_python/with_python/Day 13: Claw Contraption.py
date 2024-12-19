import time
import re
from itertools import batched


# https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
# https://www.youtube.com/watch?v=vXqlIOX2itM
def cramers_rule(button_a, button_b, prize):
    a_1, a_2 = button_a
    b_1, b_2 = button_b
    c_1, c_2 = prize

    determinant = (a_1 * b_2) - (b_1 * a_2)
    determinant_x = (c_1 * b_2) - (b_1 * c_2)
    determinant_y = (a_1 * c_2) - (c_1 * a_2)

    if determinant_x % determinant == 0 and determinant_y % determinant == 0:
        x = determinant_x // determinant
        y = determinant_y // determinant
        return (x, y)
    else:
        return (0, 0)


def calculate_tokens(presses):
    x, y = presses
    return (x * 3) + (y * 1)


def main():
    with open("../inputs/Day 13: Claw Contraption/input.txt") as file:
        input_list = [line.strip() for line in file if line.strip()]

        part_1_sum = 0
        part_2_sum = 0

        for batch in batched(input_list, 3):
            input_button_a, input_button_b, input_prize = batch

            button_a = tuple(
                map(
                    int,
                    re.match(
                        r"Button\sA:\sX\+(\d{1,2}),\sY\+(\d{1,2})", input_button_a
                    ).groups(),
                )
            )
            button_b = tuple(
                map(
                    int,
                    re.match(
                        r"Button\sB:\sX\+(\d{1,2}),\sY\+(\d{1,2})", input_button_b
                    ).groups(),
                )
            )
            prize = tuple(
                map(
                    int,
                    re.match(r"Prize:\sX=(\d+?),\sY=(\d+?)\b", input_prize).groups(),
                )
            )

            part_1_sum += calculate_tokens(cramers_rule(button_a, button_b, prize))
            part_2_sum += calculate_tokens(
                cramers_rule(
                    button_a,
                    button_b,
                    (prize[0] + 10000000000000, prize[1] + 10000000000000),
                )
            )

        print("[+] Part One Solution")
        print(
            f">>> The fewest tokens required to win all possible prizes: {part_1_sum}"
        )
        print()
        print("[+] Part Two Solution")
        print(
            f">>> The fewest tokens required to win all possible prizes: {part_2_sum}"
        )


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
