import time
from itertools import product


def isvalid_combo(key, value, combo, part2=False):
    res = 0
    operator = "+"

    for each in zip(value, combo + ("",)):
        if operator == "+":
            res += int(each[0])
        if operator == "*":
            res *= int(each[0])
        if operator == "||":
            res = int(str(res) + each[0])
        operator = each[1]

    if res == key:
        return True
    else:
        return False


def part_1(equations: dict):
    sum = 0
    operators = ["*", "+"]
    invalid_equations = {}

    for key, value in equations.items():
        isvalid = False

        for combo in product(operators, repeat=len(value) - 1):
            if isvalid_combo(key, value, combo):
                sum += key
                isvalid = True
                break

        if not isvalid:
            invalid_equations[key] = value

    print("[+] Part One Solution")
    print(f">>> Sum: {sum}")

    return (invalid_equations, sum)


def part_2(invalid_equations, sum):
    operators = ["*", "+", "||"]

    for key, value in invalid_equations.items():
        length = len(value)

        for combo in product(operators, repeat=length - 1):
            if isvalid_combo(key, value, combo, True):
                sum += key
                break

    print("[+] Part Two Solution")
    print(f">>> Sum: {sum}")


def main():
    with open("../inputs/Day 7: Bridge Repair/input.txt") as file:
        equations = {}

        for line in file.read().splitlines():
            temp = line.split(":")
            equations[int(temp[0])] = temp[1].strip().split()

        (invalid_equations, sum) = part_1(equations)
        print()
        part_2(invalid_equations, sum)


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
