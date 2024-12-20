import time
import re


def calculate_position(width, height, position, velocity, seconds):
    x = (position[0] + (velocity[0] * seconds)) % width
    y = (position[1] + (velocity[1] * seconds)) % height

    return (x, y)


def calculate_saftey_score(positions, x_middle, y_middle):
    quadrant_1 = 0
    quadrant_2 = 0
    quadrant_3 = 0
    quadrant_4 = 0

    for x, y in positions:
        if x != x_middle and y != y_middle:
            if x < x_middle and y < y_middle:
                quadrant_1 += 1
            if x > x_middle and y < y_middle:
                quadrant_2 += 1
            if x < x_middle and y > y_middle:
                quadrant_3 += 1
            if x > x_middle and y > y_middle:
                quadrant_4 += 1

    return quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4


def part_1(robots_list, width, height, x_middle, y_middle):
    positions = []

    for position, velocity in robots_list:
        positions.append(calculate_position(width, height, position, velocity, 100))

    return calculate_saftey_score(positions, x_middle, y_middle)


# https://www.youtube.com/watch?v=TO1xJg8vIMI
# https://www.youtube.com/watch?v=ySUUTxVv31U
def part_2(robots_list, width, height, x_middle, y_middle):
    safety_scores = []

    for i in range(width * height):
        positions = []

        for position, velocity in robots_list:
            positions.append(calculate_position(width, height, position, velocity, i))

        safety_scores.append(calculate_saftey_score(positions, x_middle, y_middle))

    return safety_scores.index(min(safety_scores))


def main():
    with open("../inputs/Day 14: Restroom Redoubt/input.txt") as file:
        width, height = 101, 103
        x_middle, y_middle = width // 2, height // 2

        robots_list = []

        for line in file:
            position_velocity = re.match(
                r"p=(\d{1,3}),(\d{1,3})\sv=(-?\d{1,2}),(-?\d{1,2})", line.strip()
            )

            position = (
                int(position_velocity.group(1)),
                int(position_velocity.group(2)),
            )

            velocity = (
                int(position_velocity.group(3)),
                int(position_velocity.group(4)),
            )

            robots_list.append((position, velocity))

        print("[+] Part One Solution")
        print(
            f">>> Safety factor after 100 seconds: {part_1(robots_list, width, height, x_middle, y_middle)}"
        )
        print()
        print("[+] Part Two Solution")
        print(
            f">>> The fewest number of seconds that must elapse for the robots to display the Easter egg: {part_2(robots_list, width, height, x_middle, y_middle)}"
        )


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
