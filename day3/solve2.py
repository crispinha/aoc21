def most_common(l):
	count = {"0":0, "1":0}
	for c in l:
		count[c] += 1

	if count["0"] > count["1"]:
		return "0"
	return "1"

def plif(c):
	if c == "0":
		return "1"
	return "0"

with open("ch.txt") as file:
	lines = file.readlines()
	lines = list(map(lambda s: s.strip(), lines))


	oxy = lines.copy()
	co2 = lines.copy()

	for i in range(len(lines[0])):
		all_oxy = list(map(lambda s:s[i], oxy))
		oxy_select = most_common(all_oxy)
		all_co2 = list(map(lambda s:s[i], co2))
		co2_select = plif(most_common(all_co2))
		for j, el in enumerate(lines):
			if el[i] != oxy_select:
				try:
					oxy.remove(el)
				except:
					pass
			if len(oxy) == 1:
				break

		for j, el in enumerate(lines):
			if el[i] != co2_select:
				try:
					co2.remove(el)
				except:
					pass
			if len(co2) == 1:
				break

	final_oxy = int(oxy[0], 2)
	final_co2 = int(co2[0], 2)

	print(final_oxy * final_co2)
