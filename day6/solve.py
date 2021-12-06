def clamp_to_0(i):
	if i < 0:
		return 6
	return i

def step(fishies):
	new = [f - 1 for f in fishies]
	num_to_add = len([f for f in fishies if f <= 0])
	new = [clamp_to_0(f) for f in new]
	print(num_to_add)
	new += [8]*num_to_add
	return new


def calc1(fishies):
	for _ in range(256):
		fishies = step(fishies)
	print(len(fishies))
	# print(fishies)

with open("ex.txt") as file:
	sim = [int(f) for f in file.read().split(",")]
	print(sim)
	step(sim)
	# calc1(sim)