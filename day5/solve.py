from dataclasses import dataclass
from itertools import chain
from functools import reduce

def flatten(l):
	return chain(*l)

@dataclass
class Pt:
	x: int
	y: int
	def __repr__(self):
		return f"({self.x}, {self.y})"

@dataclass
class Line:
	start: Pt
	end: Pt
	def __init__(self, st, ed):
		self.start = Pt(st[0], st[1])
		self.end = Pt(ed[0], ed[1])
	def __repr__(self):
		return f"{self.start} -> {self.end}"

def makeLine(s):
	bits = s.split(",")
	return (int(bits[0]), int(bits[1]))

def add_if_gt1(count, cell):
	if cell > 1:
		return count + 1
	return count


def make_grid():
	out = []
	for i in range(1000):
		inner = []
		for j in range(1000):
			inner.append(0)
		out.append(inner)
	return out

def calc1(lines):
	# grid = [[0]*10]*10
	grid = make_grid()
	# print(grid)

	for line in lines:
		# print(line)
		if line.start.x == line.end.x:
			if line.start.y > line.end.y:
				line.start, line.end = line.end, line.start
			for y in range(line.start.y, line.end.y+1):
				grid[line.start.x][y] += 1

		if line.start.y == line.end.y:
			if line.start.x > line.end.x:
				line.start, line.end = line.end, line.start
			for x in range(line.start.x, line.end.x+1):
				grid[x][line.start.y] += 1

	res = reduce(add_if_gt1, flatten(grid))
	# [print(x) for x in grid]
	print(res)



with open("ch.txt") as file:
	lines = []
	for fline in file.readlines():
		bits = fline.split(" -> ")
		pts = [makeLine(f) for f in bits]
		# print(pts)
		lines.append(Line(pts[0], pts[1]))
	# print(lines)

	calc1(lines)