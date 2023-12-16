import numpy as np
import pathlib

file = pathlib.Path("../input")

with open(file, "r") as fh:
    lines = list(fh.readlines())

total = 0
for line in lines:
    line = line.strip()
    cline = np.empty((len(line), ), dtype=np.int32)
    cline[:] = [ord(c) for c in line]
    nums = [
        ch - 48
        for ch in cline
        if ch >= 48 and ch <= 58
    ]
    total += nums[0]*10 + nums[-1]

print(total)
