import time
import re


def part_1(content):
    sum = 0

    for each in re.finditer(r"mul\((\d{1,3}),(\d{1,3})\)", content):
        sum += int(each.group(1)) * int(each.group(2))

    print("[+] Part One Solution")
    print(f">>> Sum of the results: {sum}")


def part_2(content):
    sum = 0
    replaced_content = content

    for match in re.finditer(r"don't\(\).+?do\(\)", content):
        replaced_content = replaced_content.replace(match.group(0), "")

    replaced_content = replaced_content.replace(
        re.search(r"don't\(\).+?$", replaced_content).group(0), ""
    )

    for each in re.finditer(r"mul\((\d{1,3}),(\d{1,3})\)", replaced_content):
        sum += int(each.group(1)) * int(each.group(2))

    print("[+] Part Two Solution")
    print(f">>> Sum of the results: {sum}")


def main():
    with open("../inputs/Day 3: Mull It Over/input.txt") as file:
        content = file.read()

        print()
        part_1(content)
        print()
        part_2(content.replace("\n", ""))


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")