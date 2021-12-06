import Debug.Trace


-- type fuckery
readInt :: String -> Int
readInt = read

pairify e = (fir, readInt sec)
    where [fir, sec] = words e

getGoodies fname = do
    contents <- readFile fname
    return (reverse (map pairify (lines contents)))

doP1 s = foldr move1 (0, 0) s

move1 ("forward", i) (x, y) = (x + i, y)
move1 ("down", i)    (x, y) = (x, y + i)
move1 ("up", i)      (x, y) = (x, y - i)

mulTup (a, b) = a * b

calc1 = fmap (mulTup . doP1) (getGoodies "ch.txt")


doP2 s = foldr move2 (0, 0, 0) s

move2 ("forward", i) (x, y, a) = (x + i, y + (a * i), a)
move2 ("down",    i) (x, y, a) = (x,     y,           a + i)
move2 ("up",      i) (x, y, a) = (x,     y,           a - i)

move3 a b = trace (show (a, b, out)) out
    where out = move2 a b 

mulTup2 (a, b, _) = a * b

calc2 = fmap (mulTup2 . doP2) (getGoodies "ch.txt")