-- type fuckery
readInt :: String -> Int
readInt = read

getGoodies fname = do
    contents <- readFile fname
    return (map (readInt) (words contents))

doP1 :: [Int] -> Int
doP1 stuff = snd (foldr p1Inner (0, 0) stuff)

p1Inner cur (lst, total)
    | cur < lst = (cur, total + 1)
    | otherwise = (cur, total)

get = getGoodies "ch1.txt"

calc1 = fmap (doP1) (getGoodies "ch1.txt")

-- doP2 :: [Int] -> Int
doP2 stuff = foldr p2Inner ((0, 0, 0), 0, 0) stuff
    -- where (start, working) = prepStuff stuff

p2Inner cur ((a, b, c), last_added, total)
    | added < last_added = ((b, c, cur), added, total + 1)
    | otherwise = ((b, c, cur), added, total)
    where added = a + b + c

-- off by one
calc2 = fmap (doP2) (getGoodies "ch1.txt")
