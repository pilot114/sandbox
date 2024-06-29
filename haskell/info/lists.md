Список - набор элементов одного типа, например, функций.
Пример функции, возвращающей список

listOfNames :: String -> [String]
listOfNames prefix =
    [prefix ++ "John", prefix ++ "Anna", prefix ++ "Andrew"]

main = print $ listOfNames "Dear "

[[Char]] это тоже самое, что [String]


Пример добавления в список:

addNewHostToFront :: String -> [String] -> [String]
addNewHostToFront newHost listOfHosts = newHost : listOfHosts

main =
    print $ addNewHostToFront "124.67.54.90" listOfHosts
    where listOfHosts = ["45.67.78.89", "123.45.65.54", "127.0.0.1"]

При этом на самом деле создался новый список.

length LIST 		- длина
name `elem` LIST 	- isset

Пример фильтрации:

removeAllEmptyNamesFrom :: [String] -> [String]
removeAllEmptyNamesFrom listOfNames =
    filter notEmptyName listOfNames
    where notEmptyName = not . null

main =
    print $ removeAllEmptyNamesFrom listOfNames
    where listOfNames = ["John", "", "Ann"]

filter использует ПРЕДИКАТ notEmptyName, являющийся композицией not . null
для фильтрации пустых значений



Сворачивание - преобразование списка в 1 значение

main =
    putStrLn $ foldl (++) "http" ["://", "www", ".", "google.com"]

Или можно создать список из всех шагов сворачивания:

main =
    print $ scanl (++) "http" ["://", "www", ".", "google.com"]

Сворачивание справа: foldr и scanr. Без начального элемента foldl1 и foldr1



Диапозоны - генерация списка в диапозоне

[1..10]
['a'..'z']
[2,4..30]
[120,110..10]
[1..]


[OPERATION ELEM | ELEM <- LIST]
или
[OPERATION ELEM | ELEM <- LIST, PREDICATES]

это генератор списка. Пример

main = print [toUpper c | c <- "http"]
main = print [toUpper c | c <- "http", c == 't']

main = print [toUpper c | c <- "http", c /= 'h', c /= 'p']

/= это неравно

Можно даже работать с несколькими списками одновременно:
[OPERATION_with_ELEMs | ELEM1 <- LIST1, ..., ELEMN <- LISTN ]

main =
    print [prefix ++ name | name <- names, prefix <- namePrefix]
    where names = ["James", "Victor", "Denis"]
        namePrefix = ["Mr. ", "sir "]

создаст ["Mr. James","sir James","Mr. Victor","sir Victor","Mr. Denis","sir Denis"]

С условием:

main =
    print [if car == "Bentley" then "Wow!" else "Good!" | car <- cars]
    where cars = ["Mercedes",
              "BMW",
              "Bentley",
              "Audi",
              "Bentley"]

Практический пример (Data.List - специальный модуль для работы со списками):

import Data.List

checkGooglerBy :: String -> String
checkGooglerBy email =
    if "gmail.com" `isSuffixOf` email
    then nameFrom email ++ " is a Googler!"
    else email
    where nameFrom fullEmail = takeWhile (/= '@') fullEmail

main = print [checkGooglerBy email | email <- ["adam@gmail.com",
                                               "bob@yahoo.com",
                                               "richard@gmail.com",
                                               "elena@yandex.ru",
                                               "denis@gmail.com"]]
