module Main where

main :: IO ()
main = do
    command <- getLine
    print $ words command

    print $ ad 3 3
    print $ de 3 3
    print $ mu 3 3
    print $ di 3 3

ad x y = x + y
de x y = x - y
mu x y = x * y
di x y = x / y
