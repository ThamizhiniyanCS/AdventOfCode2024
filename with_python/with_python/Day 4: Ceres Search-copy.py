from time import perf_counter

filename = "../inputs/Day 4: Ceres Search/input.txt"

with open(filename, mode="r") as file:
    grid = [i.replace("\n", "") for i in file.readlines()]

coordinates = []


def get_grid_bounds(grid: list[str]):
    return (len(grid), len(grid[0]))


def transpose_grid(grid: list[str]):
    max_len = max(len(s) for s in grid)
    transposed = []
    for i in range(max_len):
        row = [s[i] if i < len(s) else " " for s in grid]
        transposed.append("".join(row))
    return transposed


def horizontal_search_forward_and_backward(
    grid: list[str], bounds: tuple[int, int], word: str
):
    horizontal_word_count = 0
    # Gonna use the legendary "Sliding window technique"
    step_length = len(word)
    for index, row in enumerate(grid):
        # print(row)
        i = 0
        j = i + step_length
        while j < len(row) + 1:
            query = row[i:j]
            # print(query, i, j)
            if (query == word) or (query[::-1] == word):
                for e in range(i, j):
                    coordinates.append([index, e])
                horizontal_word_count += 1
            i += 1
            j += 1
    return horizontal_word_count


def vertical_search_forward_and_backward(
    grid: list[str], bounds: tuple[int, int], word: str
):
    # Just horizontal search a transposed grid to get the vertical stuff
    grid = transpose_grid(grid)
    bounds = get_grid_bounds(grid)

    horizontal_word_count = 0
    # Gonna use the legendary "Sliding window technique"
    step_length = len(word)
    for index, row in enumerate(grid):
        # print(row)
        i = 0
        j = i + step_length
        while j < len(row) + 1:
            query = row[i:j]
            # print(query, i, j)
            if (query == word) or (query[::-1] == word):
                for e in range(i, j):
                    coordinates.append([e, index])
                horizontal_word_count += 1
            i += 1
            j += 1

    return horizontal_word_count


def getDiagonal(lst):
    # print(lst)

    a = [lst[i][i] for i in range(len(lst))]  # DIagonal 1
    b = [lst[i][len(lst) - i - 1] for i in range(len(lst))]  # Diagonal 2
    return (a, b)


def diagonal_search_forward(grid: list[str], bounds: tuple[int, int], word: str):
    coords = []

    count = 0
    length = len(word)
    diagonals = []
    rows = len(grid)
    cols = len(grid[0])
    for col_start in range(cols):
        temp_coords = []
        diagonal = []
        row, col = 0, col_start
        while row < rows and col < cols:
            diagonal.append(grid[row][col])
            temp_coords.append([row, col])
            row += 1
            col += 1

        if len(diagonal) >= length:
            for i in range(len(diagonal) - length + 1):
                diagonals.append(diagonal[i : i + length])
                coords.append(temp_coords[i : i + length])

    for row_start in range(1, rows):
        diagonal = []
        temp_coords = []
        row, col = row_start, 0
        while row < rows and col < cols:
            diagonal.append(grid[row][col])
            row += 1
            col += 1
        if len(diagonal) >= length:
            for i in range(len(diagonal) - length + 1):
                diagonals.append(diagonal[i : i + length])
                coords.append(temp_coords[i : i + length])

    diagonals = ["".join(i) for i in diagonals]

    for index, i in enumerate(diagonals):
        if (i == word) or (i[::-1] == word):
            for each in coords[index]:
                coordinates.append(each)
            count += 1
    return count


def test_slice(m, slice_x, slice_y):
    # Thank you stackoverflow: https://stackoverflow.com/questions/48084114/how-can-i-slice-each-3x3-submatrix-from-an-arbitrary-size-matrix
    width = len(m[0])
    height = len(m)
    slices = []
    for i in range(0, height - slice_y + 1):
        for j in range(0, width - slice_x + 1):
            slices.append(
                [
                    [m[a][b] for b in range(j, j + slice_x)]
                    for a in range(i, i + slice_y)
                ]
            )
    return slices


def x_mas(grid: list[str], bounds: tuple[int, int], word: str):
    # Now we can use the legendary 2d sliding window
    count = 0
    slices = test_slice(grid, len(word), len(word))
    for matrix in slices:
        word_a = "".join([matrix[i][i] for i in range(len(matrix))])
        word_b = "".join([matrix[i][len(matrix) - i - 1] for i in range(len(matrix))])
        if ((word_a == word) or word_a[::-1] == word) and (
            (word_b == word) or (word_b[::-1]) == word
        ):
            count += 1
    return count


def diagonal_search_backward(grid: list[str], bounds: tuple[int, int], word: str):
    grid = [i[::-1] for i in grid]
    coords = []

    count = 0
    length = len(word)
    diagonals = []
    rows = len(grid)
    cols = len(grid[0])
    for col_start in range(cols):
        temp_coords = []
        diagonal = []
        row, col = 0, col_start
        while row < rows and col < cols:
            diagonal.append(grid[row][col])
            temp_coords.append([row, col])
            row += 1
            col += 1

        if len(diagonal) >= length:
            for i in range(len(diagonal) - length + 1):
                diagonals.append(diagonal[i : i + length])
                coords.append(temp_coords[i : i + length])

    for row_start in range(1, rows):
        diagonal = []
        temp_coords = []
        row, col = row_start, 0
        while row < rows and col < cols:
            diagonal.append(grid[row][col])
            row += 1
            col += 1
        if len(diagonal) >= length:
            for i in range(len(diagonal) - length + 1):
                diagonals.append(diagonal[i : i + length])
                coords.append(temp_coords[i : i + length])

    diagonals = ["".join(i) for i in diagonals]

    for index, i in enumerate(diagonals):
        if (i == word) or (i[::-1] == word):
            for each in coords[index]:
                coordinates.append([each[1], each[0]])
            count += 1
    return count


def part1():
    bounds = get_grid_bounds(grid)

    word = "XMAS"
    print(
        horizontal_search_forward_and_backward(grid, bounds, word)
        + vertical_search_forward_and_backward(grid, bounds, word)
        + diagonal_search_forward(grid, bounds, word)
        + diagonal_search_backward(grid, bounds, word)
    )

    print(f"    0 1 2 3 4 5 6 7 8 9")

    dotted_map = [["."] * bounds[1] for _ in range(bounds[0])]

    # print(grid)

    for each in coordinates:
        dotted_map[each[0]][each[1]] = grid[each[0]][each[1]]

    for index, each in enumerate(dotted_map):
        print(f"{str(index).ljust(3)} {" ".join(each)}")
    
    print(len(coordinates))




def part2():
    print(x_mas(grid, get_grid_bounds(grid), "MAS"))


if __name__ == "__main__":
    s = perf_counter()
    part1()
    # print(coordinates)
    # part2()
    e = perf_counter()
    print(f"[*] Time: {e - s}")
