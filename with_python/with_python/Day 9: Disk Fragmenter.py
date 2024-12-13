import time
from collections import Counter


def part_1(file_system):
    file_system_copy = list(file_system)
    filesystem_checksum = 0

    while None in file_system_copy:
        try:
            file_system_copy[file_system_copy.index(None)] = file_system_copy.pop()
        except:
            continue

    for i, id in enumerate(file_system_copy):
        filesystem_checksum += i * id

    print("[+] Part One Solution")
    print(f">>> File System Checksum: {filesystem_checksum}")


def part_2(file_system, character_occurances: dict):
    file_system_copy = list(file_system)
    len_file_system_copy = len(file_system_copy)
    filesystem_checksum = 0

    file_ids = [i for i in character_occurances.keys() if i != None]

    for id in reversed(file_ids):
        id_start_index = file_system_copy.index(id)

        file_indexes = [
            i
            for i in range(
                id_start_index,
                id_start_index + character_occurances[id],
            )
        ]

        empty_spaces = []

        for i in range(file_system_copy.index(None), len_file_system_copy):
            if file_system_copy[i] == None:
                empty_spaces.append(i)
            else:
                if len(empty_spaces) >= character_occurances[id]:
                    break
                else:
                    empty_spaces = []

        for file, space in zip(file_indexes, empty_spaces):
            if space < file:
                file_system_copy[space] = file_system_copy[file]
                file_system_copy[file] = None

    for i, id in enumerate(file_system_copy):
        if id != None:
            filesystem_checksum += i * int(id)

    print("[+] Part Two Solution")
    print(f">>> File System Checksum: {filesystem_checksum}")


def main():
    with open("../inputs/Day 9: Disk Fragmenter/input.txt") as file:
        disk_map_string = file.read().strip()
        len_disk_map_string = len(disk_map_string)
        file_system = []
        pairs = []

        for i in range(0, len_disk_map_string, 2):
            pairs.append(
                [
                    disk_map_string[i],
                    disk_map_string[i + 1] if i < len_disk_map_string - 1 else "",
                ]
            )

        for i, pair in enumerate(pairs):
            if i < len(pairs) - 1:

                for _ in range(int(pair[0])):
                    file_system.append(i)

                for _ in range(int(pair[1])):
                    file_system.append(None)
            else:

                for _ in range(int(pair[0])):
                    file_system.append(i)

        character_occurences = Counter(file_system)

        part_1(file_system)
        print()
        part_2(file_system, character_occurences)


if __name__ == "__main__":
    print()

    start_time = time.perf_counter()
    main()
    end_time = time.perf_counter()

    print(f"\nTotal Time Taken: {end_time - start_time} seconds.")
