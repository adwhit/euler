singles = 25:[20,19..1] :: [Int]
doubles = 50:[40,38..1] :: [Int]
triples =    [60,57..1] :: [Int]
poss = triples ++ doubles ++ singles :: [Int]

allScrs :: [Int] -> [Int]
allScrs scrs = [ x + z | x <- scrs, z <- doubles, x + z < 100]

totScr = (length . allScrs . scores $ pairs poss)  +  (length $ allScrs poss) + 21

pairs :: [Int] -> [(Int, [Int])]
pairs []     = []
pairs a@(x:xs) = (x,a) : pairs xs

scores :: [(Int, [Int])] -> [Int]
scores []          = []
scores ((x,ys):xs) = combos x ys ++ scores xs
  where
    combos x' ys' = [x' + y | y <- ys']

main :: IO ()
main = print $ totScr
