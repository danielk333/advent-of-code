import numpy as np
import re

TEST_DATA = '''
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
'''

def read_data(lines):
    rows = []
    locations = None

    # structure (amount, from, to)
    instructions = []
    
    # we have three parts, creates, locations and instructions and we start with creates
    reading_crates = True
    for line in lines:
        if len(line) == 0:
            continue
        
        if reading_crates:
            row_inds = [m.start(0) for m in re.finditer('\[', line)]
            row_data = [line[ind + 1] for ind in row_inds]

            #translate row index to location index
            row_inds = [ind//4 + 1 for ind in row_inds]

            if len(row_inds) == 0:
                locations = int(line.strip().split()[-1])
                reading_crates = False
            else:
                rows.append((row_inds, row_data))
        else:
            parts = line.split()
            instructions.append([int(parts[ind]) for ind in [1, 3, 5]])

    #restructre the data
    crates = {}
    
    return crates, instructions

if __name__ == '__main__':
    crates, instructions = read_data(TEST_DATA.split('\n'))