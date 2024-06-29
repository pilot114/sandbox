L функция

\x -> x * x

Применим её к данным:
(\x -> x * x) 5
>25

(\x y -> x * y) 5 10
>50


Функция высшего порядка - это "функция функций".
Она может принимать функции как аргументы и возвращать функцию.


Каррирование

НА САМОМ деле, чистая функция принимает 1 аргумент который КАРРИРУЕТСЯ из нескольких.
Отсюда и такой синтаксис -> -> ->

Пример:
    main = print (divide 10.03 2.1)
Означает
    1) divide 10.03             => Double -> Double
    2) Double -> Double -> 2.1  => 4.77

Зачем это нужно? для частичного применения функции!

let temp = divide 10.03
temp 2.1

Отсюда вывод: любая функция с более чем 1 аргументом, является функцией высшего порядка.


Цепочки функций.

	module Main where

	import Data.Char
	import Data.List
	import Data.String.Utils

	addPrefix :: String -> String
	addPrefix url =
	    if prefix `isPrefixOf` url then url else prefix ++ url
	    where prefix = "http://"

	encodeAllSpaces = replace " " "%20"
	makeItLowerCase = map toLower

	main =
	    putStrLn (addPrefix (encodeAllSpaces (makeItLowerCase url)))
	    where url = "www.SITE.com/test me/Start page"

фактически, всё свелось к строке addPrefix (encodeAllSpaces (makeItLowerCase url)),
приводящей url к нужному виду.
Это выражение можно ещё упростить, используя КОМПОЗИЦИЮ функций:

(addPrefix . encodeAllSpaces . makeItLowerCase) url
или так (ПРИМЕНЕНИЕ функций):
addPrefix $ encodeAllSpaces $ makeItLowerCase url

Второй вариант работает и с обычными аргументами, поэтому в итоге обычно используют чтото вроде:

addPrefix.encodeAllSpaces.makeItLowerCase $ url


Операторы: да, можно мутить свои операторы

(+-+) :: String -> String -> String
str1 +-+ str2 = str1 ++ "-" ++ str2

main = putStrLn $ "Hi, haskeller!" +-+ " You're awesome!"
