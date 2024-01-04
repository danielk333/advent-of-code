import re
from pprint import pprint
file = "input"
sep = "."
not_syms = [str(x) for x in range(10)] + [sep]
    

with open(file, "r") as fh:
    lines = list(fh.readlines())

shape = (len(lines), len(lines[0]) - 1)

numbers = []
parts = {}
number_regex = re.compile(r"[0-9]+")

def get_border_coords(y, start, end, size):
    inds = []
    if start > 0:
        inds += [(y + 0, start - 1)]
        if y < size[0] - 1: inds += (y + 1, start - 1),
        if y > 0: inds += (y - 1, start - 1),
    if end < size[1]:
        inds += [(y + 0, end)]
        if y < size[0] - 1: inds += [(y + 1, end)]
        if y > 0: inds += [(y - 1, end)]
    if y < size[0] - 1:
        inds += [(y + 1, ind) for ind in range(start, end)]
    if y > 0:
        inds += [(y - 1, ind) for ind in range(start, end)]
    return inds


def test_border_select(y, x, width):
    print("\n")
    test_lines = [["-"]*100 for x in range(4)]
    for xi in range(x, x + width):
        test_lines[y][xi] = "."
        
    for line in test_lines:
        print("".join(line))
    print("\n")
    test_c = get_border_coords(y, x, x + width, (4, 100))
    for (yi, xi) in test_c:
        test_lines[yi][xi] = "*"

    for line in test_lines:
        print("".join(line))

# test_border_select(2, 20, 4)
# test_border_select(0, 40, 10)
# test_border_select(3, 60, 1)
# test_border_select(2, 0, 4)
# test_border_select(2, 95, 5)
# exit()

for ind, line in enumerate(lines):
    numbers += [
        # row, start column, end column, number
        (ind, m.start(0), m.end(0), int(line[m.start(0):m.end(0)]))
        for m in number_regex.finditer(line)
    ]

# print(numbers)
# exit()

include = [False]*len(numbers)
for ind, (y, x0, x1, num) in enumerate(numbers):
    border_coords = get_border_coords(y, x0, x1, shape)

    for by, bx in border_coords:
        if lines[by][bx] in not_syms:
            continue

        if by not in parts: parts[by] = {}
        if bx not in parts[by]: parts[by][bx] = (lines[by][bx], [])
        parts[by][bx][1].append(num)
        include[ind] = True

# pprint(parts)

# Analyse data
total_nums = 0
for use, data in zip(include, numbers):
    if use:
        total_nums += data[3]

print(f"Part 1: {total_nums}")

total_gear_ratios = 0
for y in parts:
    for x in parts[y]:
        part, nums = parts[y][x]
        if part != "*" or len(nums) != 2:
            continue
        # print(part, nums)
        total_gear_ratios += nums[0]*nums[1]

print(f"Part 2: {total_gear_ratios}")
