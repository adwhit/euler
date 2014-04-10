generate :: Int -> Int
generate x = sum $ map (\y -> x^y * (-1)^y) [0..10]

uns = map generate [1..10]

diff :: [Int] -> [Int]
diff (x:y:xs) = (y-x):diff(y:xs)
diff [y]    = []

foldlist :: [Int] -> Int
foldlist [x] = x
foldlist xs  = foldlist (diff xs)

fact :: Int -> Int
fact 0 = 1
fact n = n * fact (n-1)

getFactor :: [Int] -> Int
getFactor arr = foldlist arr `div`  (fact $ (length arr) -1)

removeDegree :: Int -> Int -> [Int] -> [Int]
removeDegree deg coef arr = map (\(x,y) -> x - coef*(y^deg)) (zip arr [1..])

getPoly :: [Int] -> [Int]
getPoly [] = []
getPoly arr = coef:(getPoly (removeDegree deg coef (init arr)))
  where
    deg  = (length arr) - 1
    coef = getFactor arr

nextVal :: [Int] -> Int
nextVal xs = sum $ zipWith (\x y -> x*(n^y)) (reverse xs) [0..]
  where n = length xs + 1

getFits :: [Int] -> [Int]
getFits arr = tail $ map (nextVal . getPoly) (reverse $tails arr)

tails :: [Int] -> [[Int]]
tails [] = [[]]
tails y = y:tails (init y)

main = print $ sum $ getFits uns
