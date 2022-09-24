module Main where

main :: IO ()
main = putStrLn "Hello, Haskell!"

and' :: [Bool] -> Bool
and' [] = True
and' (x:xs) | not x = False 
            | otherwise = and' xs

concat' :: [[a]] -> [a]
concat' [] = []
concat' (x:xs) = x ++ concat' xs

replicate' :: Int -> a -> [a]
replicate' 0 x = []
replicate' n x = x : replicate' (n - 1) x

insertSort :: Ord a => [a] -> [a]
insertSort [] = []
insertSort (x:xs) = insert x (insertSort xs)
  where
    insert :: Ord a => a -> [a] -> [a]
    insert x [] = [x]
    insert x (y:ys) | x <= y = x : y : ys
                    | otherwise = y : insert x ys

mergeSort :: Ord a => [a] -> [a]
mergeSort [] = []
mergeSort [x] = [x]
mergeSort xs = merge (mergeSort ys) (mergeSort zs)
  where
    (ys, zs) = half xs
    half xs = splitAt (length xs `div` 2) xs

    merge :: Ord a => [a] -> [a] -> [a]
    merge xs [] = xs
    merge [] ys = ys
    merge (x:xs) (y:ys) | x <= y = x : merge xs (y:ys)
                        | otherwise = y : merge (x:xs) ys

quickSort :: Ord a => [a] -> [a]
quickSort [] = []
quickSort (x:xs) = quickSort smaller ++ [x] ++ quickSort larger
  where
    smaller = [a | a <- xs, a <= x]
    larger  = [b | b <- xs, b > x]
