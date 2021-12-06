import pygame
from dataclasses import dataclass
from pygame.locals import *
pygame.init()

view = True

@dataclass
class Line:
	start: (int, int)
	end: (int, int)
	# def __init__(self, st, ed):
	# 	self.start = Pt(st[0], st[1])
	# 	self.end = Pt(ed[0], ed[1])
	def __repr__(self):
		return f"{self.start} -> {self.end}"




def makeLine(s):
	bits = s.split(",")
	return (int(bits[0]), int(bits[1]))

lines = []
with open("ch.txt") as file:
	
	for fline in file.readlines():
		bits = fline.split(" -> ")
		pts = [makeLine(f) for f in bits]
		# print(pts)
		lines.append(Line(pts[0], pts[1]))

# print(lines)

screen = pygame.display.set_mode((1000, 1000))
screen.fill((0, 0, 0))

surf = pygame.Surface((1000, 1000), SRCALPHA) 
surf.fill((0, 0, 0))

tempsurf = pygame.Surface((1000, 1000), SRCALPHA) 


for line in lines:
	tempsurf.fill((0, 0, 0))
	pygame.draw.line(tempsurf, (1, 255, 255, 1), line.start, line.end)
	surf.blit(tempsurf, (0, 0), special_flags=BLEND_RGBA_ADD)

count = 0
for x in range(1000):
	for y in range(1000):
		col = surf.get_at((x, y))
		if col.r > 1 :
			count += 1
print(count)

while view:
	# screen.fill((0, 0, 0))
	screen.blit(surf, (0, 0))
	# 
	# screen.fill((255, 255, 255))
	# pygame.draw.circle(screen, (0.5, 0, 0.5), )
	# for line in lines:
	# 	pygame.draw.line(screen, (1, 1, 1, 1), line.start, line.end)
	pygame.display.update()

	