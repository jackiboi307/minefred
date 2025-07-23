#!/bin/python

import re
import os
import sys
import json

def err(msg=None):
	print(f"Error occured!")
	if msg:
		print(f"{msg}")
	print("Exiting.")
	sys.exit(1)

def parse_hex(hex_color):
    hex_color = hex_color.lstrip('#')
    
    match len(hex_color):
    	case 1:
    		hex_color *= 6
    		hex_color += "ff"
    	case 2:
    		hex_color = hex_color[0]*6+hex_color[1]*2
    	case 3:
    		hex_color = \
    			hex_color[0]*2 + \
    			hex_color[1]*2 + \
    			hex_color[2]*2
    		hex_color += "ff"
    	case 4:
    		hex_color = \
    			hex_color[0]*2 + \
    			hex_color[1]*2 + \
    			hex_color[2]*2 + \
    			hex_color[3]*2
    	case 6:
    		hex_color += "ff"
    
    return tuple(int(hex_color[i:i + 2], 16) for i in (0, 2, 4, 6))
    
def parse_palette(path=None, start_palette=None, text=None):
	if not text:
		with open(path) as file:
			text = file.read()
		
	palette = start_palette if start_palette else \
		{" ": (0, 0, 0, 0)}
	
	for num, line in enumerate(text.splitlines()):
		if line == "" or line.isspace() or \
				line.lstrip().startswith("//"):
			pass
		elif m := re.fullmatch(r"(#(?:[A-Fa-f0-9]{1,4}|[A-Fa-f0-9]{6}|[A-Fa-f0-9]{8}))\s+(\S+)", line):
			palette[m.group(2)] = parse_hex(m.group(1))
		elif m := re.fullmatch(r"(\S+)\s+(\S+)", line):
			palette[m.group(1)] = palette[m.group(2)]
		elif m := re.fullmatch(r"(#(?:[A-Fa-f0-9]{1,4}|[A-Fa-f0-9]{6}|[A-Fa-f0-9]{8}))", line):
			palette[" "] = parse_hex(m.group(1))
		elif m := re.fullmatch(r"(\S+)", line):
			palette[" "] = palette[m.group(1)]
		else:
			err(f"Syntax error in {path} (row {num})\n    {line}")
	
	return palette
	
def parse_texture(path, palette):
	with open(path) as file:
		text = file.read()
		
	spl = text.split("\n=\n", 1)
	if len(spl) == 2:
		text, tail = spl
		palette = parse_palette(text=tail, start_palette=palette)
		
	lines = text.splitlines()
		
	return [
		[palette[c] for c in \
		line+" "*(len(lines[0])-len(line))] for \
		line in lines
	]
	
def parse_dir(path, palette=None):
	if os.path.isfile(os.path.join(path, "palette.txt")):
		palette = parse_palette(
			os.path.join(path, "palette.txt"), palette)
		
	textures = {}
	
	for name in os.listdir(path):
		if os.path.isfile(os.path.join(path, name)):
			if name == "palette.txt":
				pass
			elif m := re.fullmatch(r"([a-z0-9_]+)\.txt", name):
				textures[m.group(1)] = \
					parse_texture(
						os.path.join(path, name),
						palette
					)
			else:
				pass
		elif os.path.isdir(os.path.join(path, name)):
			textures = textures | \
				parse_dir(os.path.join(path, name), palette)
	
	return textures
	
if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"usage: {sys.argv[0]} <source> <dest>")
        sys.exit(1)

    source = sys.argv[1]
    dest   = sys.argv[2]

    print(f"Parsing...")

    textures = parse_dir(source)

    print("Done.\n")

    for texture in textures:
        print(f"{texture}")
        for row in textures[texture]:
            print("".join([
                "\x1b[48;2;{};{};{}m  ".format(*col[:3])
                for col in row
            ]) + "\033[0m")
        print()

    print(f"Writing to {dest}...")

    with open(dest, "w") as file:
        json.dump(textures, file)
        
    print("Done.")
