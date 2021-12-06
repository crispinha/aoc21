import Data.List

readInt :: String -> Int
readInt = read

getGoodies fname = do
    contents <- readFile fname
    return (transpose (lines contents))

get = getGoodies "ex.txt"

mostCommon l
    | zeros > ones = '0'
    | otherwise    = '1'
    where (zeros, ones) = mostCommon' l (0, 0)

mostCommon' :: String -> (Int, Int) -> (Int, Int)
mostCommon' [] (zeros, ones)         = (zeros, ones)
mostCommon' ('0' : ls) (zeros, ones) = mostCommon' ls (zeros + 1, ones)
mostCommon' ('1' : ls) (zeros, ones) = mostCommon' ls (zeros, ones + 1)

plif '0' = '1'
plif '1' = '0'

gamma i   = fmap (map mostCommon) i
epsilon i = fmap (map plif . map mostCommon) i

-- stolen from so
bintodec :: Integral i => i -> i
bintodec 0 = 0
bintodec i = 2 * bintodec (div i 10) + (mod i 10)

binify :: IO String -> IO Int
binify i = fmap (bintodec . readInt) i

calc1 = do 
    gam <- binify (gamma (getGoodies "ch.txt"))
    eps <- binify (epsilon (getGoodies "ch.txt"))
    return (gam * eps)
