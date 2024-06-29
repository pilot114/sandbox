{-# LANGUAGE MultiWayIf #-}

module Main where

test :: Int -> String
test s =
    if | s == 10 -> '10'
       | s == 9 -> '9'
       | s == 8 -> '8'
       | overwise -> 'i dont know'

Папам, это case на охранниках. Чтобы работало,
используется директива для компилятора "LANGUAGE MultiWayIf"

Но есть и другой способ:

test :: Int -> String
test s
    | s == 10 = '10'
    | s == 9 = '9'
    | s == 8 = '8'
    | overwise = 'i dont know'

И, наконец, никто не отменял сравнение с образцом:

test :: Int -> String
test 10 = '10'
test 9 = '9'
test 8 = '8'
test _ = 'i dont know'
