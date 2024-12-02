import time


def is_safe_report(report) -> bool:
    order = ""

    for i in range(len(report) - 1):
        a = report[i]
        b = report[i + 1]

        if 1 <= abs(a - b) <= 3:
            if not order:
                if a > b:
                    order = ">"
                else:
                    order = "<"

            if order == "<" and not a < b:
                return False

            if order == ">" and not a > b:
                return False

        else:
            return False

    return True


def part_1(file):
    total_safe_reports = 0

    unsafe_reports = []

    for line in file:
        report = list(map(int, line.strip().split()))

        if is_safe_report(report):
            total_safe_reports += 1
        else:
            unsafe_reports.append(report)

    print("[+] Part One Solution")
    print(f">>> Total Safe Reports: {total_safe_reports}")

    return [total_safe_reports, unsafe_reports]


def part_2(total_safe_reports: int, unsafe_reports: list):
    for report in unsafe_reports:
        for i in range(len(report)):
            report_copy = report.copy()
            report_copy.pop(i)

            if is_safe_report(report_copy):
                total_safe_reports += 1
                break

    print("[+] Part Two Solution")
    print(f">>> Total Safe Reports after implementing Problem Dampener: {total_safe_reports}")


def main():
    with open("../inputs/Day 2: Red-Nosed Reports/input.txt") as file:
        print()
        [total_safe_reports, unsafe_reports] = part_1(file)
        print()
        part_2(total_safe_reports, unsafe_reports)


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
