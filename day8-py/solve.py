def h(s):
	o = ""
	for c in s:
		o += c
	return o

def flip(d):
	return {v: k for k, v in d.items()}

def of_len(iterab, n):
	return [x for x in iterab if len(x) == n]

def find_letter_freqs(data):
	o = {}
	for s in data:
		for c in s:
			count = o.get(c, 0)
			o[c] = count + 1
	return o

def build_mapping(raw):
	working_data = raw.copy()
	letter_freqs = find_letter_freqs(working_data)
	to_del = []
	mapping = {}
	letter_map = {}

	# first pass on length
	for datum in working_data:
		if len(datum) == 2:
			mapping[h(datum)] = "1"
			to_del.append(datum)
		elif len(datum) == 3:
			mapping[h(datum)] = "7"
			to_del.append(datum)
		elif len(datum) == 4:
			mapping[h(datum)] = "4"
			to_del.append(datum)
		elif len(datum) == 7:
			mapping[h(datum)] = "8"
			to_del.append(datum)
	for delo in working_data:
		working_data.remove(delo)
	to_del = []

	flipped_lfs = flip(letter_freqs)
	letter_map["b"] = flipped_lfs[6]
	letter_map["e"] = flipped_lfs[4]
	letter_map["f"] = flipped_lfs[9]

	flipped_map = flip(mapping)
	letter_map["c"] = (set(flipped_map["1"]) - set(letter_map["f"])).pop()
	letter_map["a"] = (set(flipped_map["7"]) - {letter_map["f"], letter_map["c"]}).pop()
	letter_map["d"] = (set(flipped_map["4"]) - {letter_map["b"], letter_map["f"],
												letter_map["c"]}).pop()
	letter_map["g"] = (set(flipped_map["8"]) - {letter_map["a"], letter_map["b"],
											 letter_map["c"], letter_map["d"],
											 letter_map["e"], letter_map["f"]}).pop()

	for thing in of_len(raw, 6):
		if letter_map["e"] not in thing:
			mapping[h(thing)] = "9"

	zero = letter_map["a"] + letter_map["b"] + letter_map["c"] + letter_map["e"] + letter_map["f"] + letter_map["g"]
	mapping[zero] = "0"

	two = letter_map["a"] + letter_map["c"] + letter_map["d"] + letter_map["e"] + letter_map["g"]
	mapping[two] = "2"

	three = letter_map["a"] + letter_map["c"] + letter_map["d"] + letter_map["f"] + letter_map["g"]
	mapping[three] = "3"

	five = letter_map["a"] + letter_map["b"] + letter_map["d"] + letter_map["f"] + letter_map["g"]
	mapping[five] = "5"

	six = letter_map["a"] + letter_map["b"] + letter_map["d"] + letter_map["e"] + letter_map["f"] + letter_map["g"]
	mapping[six] = "6"

	eight = letter_map["a"] + letter_map["b"] + letter_map["c"] + letter_map["d"] + letter_map["e"] + letter_map["f"] + letter_map["g"]
	mapping[eight] = "8"

	mapping = {"".join(sorted(k)): v for k, v in mapping.items()}

	return mapping

def decode(mapping, c):
	char = "".join(sorted(c))
	return mapping[char]

def calc2(data):
	total = 0
	for datum in data:
		mapping = build_mapping(datum[0])
		running = ""
		for c in datum[1]:
			running += decode(mapping, c)
		total += int(running)
	print(total)

with open("ch.txt") as file:
	lines = file.readlines()
	data = []
	for line in lines:
		mapping, digits = line.split("|")
		data.append((mapping.split(), digits.split()))
	calc2(data)
	