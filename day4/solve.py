from functools import reduce
from itertools import chain

def lmap(f, l):
	return list(map(f, l))

def split_on_nl(xs):
	stuff = [[]]
	for x in xs:
		if x == "\n":
			stuff.append([])
			continue
		stuff[-1].append(x)

	return stuff

def p_boards(bs):
	for b in bs:
		for l in b:
			print(l)
		print()

def mark_cell(cell, call):
	(num, res) = cell
	if num == call:
		return (num, True)
	return cell

def mark(bs, call):
	for i, _ in enumerate(bs):
		bs[i] = lmap(lambda col: lmap(lambda cel: mark_cell(cel, call), col), bs[i])

def check_if_wins(board):
	# horizontal
	for row in board:
		winning_row = True
		for c in row:
			if c[1] == False:
				winning_row = False
		if winning_row:
			# print("doublefuck")
			return True

	# list transpose nicked from so again
	for col in list(map(list, zip(*board))):
		winning_col = True
		for c in col:
			if c[1] == False:
				winning_col = False
		if winning_col:
			# print("fuck")
			return True
	return False

def num_if_false(cell):
	(num, res) = cell
	if res == False:
		return num
	return 0


def calc1(calls, boards):
	for c in calls:
		mark(boards, c)

		for board in boards:
			if check_if_wins(board):
				flatboard = chain(*board)
				total = reduce(lambda l, r: l + num_if_false(r), flatboard, 0)
				p_boards([board])
				# print(list(flatboard))
				# print(board)
				print(total, c)
				print(total * c)
				return

def calc2(calls, boards):
	final = None
	last = None
	for i, c in enumerate(calls):
		mark(boards, c)

		for board in boards.copy():
			if check_if_wins(board):
				boards.remove(board)
		if len(boards) == 1:
			final = boards[0]
			last = i
			break

	last += 1
	while not check_if_wins(boards[0]):
		mark(boards, calls[last])
		last += 1

	flatboard = chain(*boards[0])
	total = reduce(lambda l, r: l + num_if_false(r), flatboard, 0)
	print(total, calls[last-1])
	print(total * calls[last-1])



with open("ch.txt") as file:
	lines = file.readlines()

	calls = lmap(int, lines[0].split(","))
	rest = split_on_nl(lines[2:])
	rest = lmap(lambda s: lmap(lambda t: t.split(), s), rest)
	boards = lmap(lambda s: lmap(lambda t: lmap(lambda u: (int(u), False), t), s), rest)

	print(calls)
	print()

	# calc1(calls, boards)
	calc2(calls, boards)
