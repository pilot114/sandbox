Кортеж - это список, в котором могут быть элементы разных типов.
Из кортежа можно только читать, обычно кортежи состоят из 2 элементов.

(123, 'Test')

chessMove :: (String, String) -> String
chessMove pair = fst pair ++ "-" ++ snd pair

main = print $ chessMove ("e2", "e4")


Читаем элемент:

import Data.Tuple.Select
main = print $ sel3 ("One", "Two", "Three", "Four")

Тип кортежа зависит от количества элементов в нём

Создаются элементы обычным матчингом
tuple x y = (x, y)
let (a, b) = tuple "a" "b"
