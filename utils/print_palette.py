#!/bin/python

import parse
import sys

if len(sys.argv) != 2:
	print(f"usage: {sys.argv[0]} <source>")
	sys.exit(1)

source = sys.argv[1]
palette = parse.parse_palette(source)

RESET = "\x1b[0m"

for color in palette:
    col = "\x1b[48;2;{};{};{}m".format(*palette[color][:3])
    name = color if color != " " else "' ' (space)"
    print(f"{col}    {RESET} {name}\n{col}    {RESET}")

