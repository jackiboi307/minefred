#!/bin/python

import sys
import random

if len(sys.argv) not in (4, 5):
	print(f"usage: {sys.argv[0]} <width>x<height> <input> <dest> [end]")
	sys.exit(1)

width, height = sys.argv[1].split("x", 1)
width = int(width)
height = int(height)
inp = sys.argv[2]
dest = sys.argv[3]
end = "\n=\n" + sys.argv[4] if len(sys.argv) == 5 else ""

with open(dest, "w") as file:
    file.write("\n".join(["".join([
        random.choice(inp) for _ in range(width)
    ]) for _ in range(height)]) + end)
