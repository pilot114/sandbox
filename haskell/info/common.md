Реп с примерами:
https://github.com/denisshevchenko/ohaskell-code


Haskell, будучи универсальным языком, прекрасно подходит для самых разных сегментов ПО. Это и клиент-серверное программирование, и консольные утилиты, и десктопные приложения с GUI, и мобильные приложения, и даже веб-разработка. Причём Haskell не просто может использоваться в этих областях, но использоваться эффективно.

imperative - набор инструкций выполняется строго последовательно, есть оператор присваивания
declarative - описание конечного результата. Мы не можем заранее определять, в каком порядке
будут выполняться функции.


Haskell ленивый.
Благодаря лени, replicate создаст лишь 2 копии строки "127.0.0.1":
main = print (take 2 (replicate 100 "127.0.0.1"))
вариант а-ля yield:
main = print (take 2 (repeat "127.0.0.1"))
Также, если функция не использует один из своих аргументов, то и вычислятся(если он вычисляется), он не будет.
другой пример лени:
let k = \x y -> x
k 42 undefined
выведет 42. undefined- особая функция, которая всегда расходится
(вызывает ошибку). тут все прошло гладко, т.к. второе
значение не юзается, соответственно и не выполняется



Haskell надежный.
Если прога скомпилилась, она гарантированно работает. Типизация строгая, но при этом автоматическая.

Haskell чистый.
Чистая функция - это математическая функция, с независимым ни от чего, кроме аргументов, поведением.
Фактически, чистые функции работают только с примитивными типами.

simpleSum :: Int -> Int -> Int
simpleSum valueOne, valueTwo = valueOne + valueTwo
main = putStrLn (show (simpleSum 4 8))


Примеры чистых функций:
indicate :: String -> String
indicate address =
 if address == "127.0.0.1" then "localhost" else address


на php это выглядело бы так:
function($address){
	if($address == '127.0.0.1'){
		return 'localhost';
	} else {
		return $address;
	}
}

Или, вместо if-the-else:
indicate :: String -> String
indicate "127.0.0.1" = "localhost"
indicate address = address

Тут мы видим, что у одной функции может быть несколько реализаций!
По сути, это перегрузка функции и выбор реализации зависит от входных параметров.



Guard - интересная возможость в Haskell. Пример:

indicate :: String -> String
indicate address
    | address == "127.0.0.1" = "localhost"
    | null address = "empty IP-address"
    | otherwise = address

| это 'ИЛИ', затем идет логическое выражение, затем результат
otherwise - это фактически default




Для избавления от магических чисел можно использовать локальные выражения:

prepareLength :: Double -> Double
prepareLength line =
    line * coefficient - correction
    where coefficient = 0.4959
          correction = 0.0012

where может быть только одно и только в конце тела функции.
Но то же можно сделать с помощью ключевого слова 'let':

prepareLength :: Double -> Double
prepareLength line =
    let coefficient = 12.4959
        correction = 0.0012
    in
    line * coefficient - correction

Также let позволяет создавать супер-локальные выражения (существуют в пределах
текущих круглых скобок)

предикат - функция, возвращающая boolean
