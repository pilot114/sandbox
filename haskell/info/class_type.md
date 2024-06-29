data - ключевое слово для определения типа
class - класс типов. Логически объединенная группа заготовок методов. (== абстрактный интерфейс)
instance - экземпляр класса типов (т.е. реализация)


Объявление и конструктор типа

data IPAddress = IPAddress String
let localhost = IPAddress "127.0.0.1"


Полиморфное объявление ("a" - любого типа) и реализация класса (где "a" уже конкретного типа)

class Show a where
    show :: a -> String

instance Show IPAddress where
    show (IPAddress address) =
        if address == "127.0.0.1" then "localhost" else address


здесь используется pattern matching - единственный способ извлечения значения из класса
(строка address какбы порождает IPAddress и паралельно используется в выражении)

По сути тут нет наследования, только имплементация абстрактного интерфейса.
Получается что наследовать реализацию нельзя.



value constructor - псевдоним типа для значения, передаваемого
в конструктор

data IPAddress = IP String
instance Show IPAddress where
	show (IP address) =
		if address == "127.0.0.1" then "localhost" else address

Применение:

main = putStrLn . show $ IP "127.0.0.1"

Конструкторов может быть несколько, для этого используем перегрузку. Пример:
data IPAddress = IP String | Host String

затем две реализации и использование:

instance Show IPAddress where
    show (IP address)   = address
    show (Host address) = if address == "127.0.0.1" then "localhost" else address

main = putStrLn . show $ IP "127.0.0.1"
main = putStrLn . show $ Host "127.0.0.1"


data IPAddress = Host String Int - конструктор c 2 параметрами, например


Нуальный конструктор: не принимает аргумент, но может использоваться как перечисляемый тип:

data TransportLayer = TCP | UDP | SCTP | DCCP | SPX

descriptionOf :: TransportLayer -> String
descriptionOf protocol =
    case protocol of
        TCP  -> "Transmission Control Protocol"
        UDP  -> "User Datagram Protocol"
        SCTP -> "Stream Control Transmission Protocol"
        DCCP -> "Datagram Congestion Control Protocol"
        SPX  -> "Sequenced Packet Exchange"

Это типичное использование нуального конструктора



Ограничение типов(контекст):

test :: Eq a => a -> Bool

Функция test работает только с "а" имеющим тип Eq
(т.е. реализующий операции сравнения)

Несколько контекстов:
nothing :: (Show a, Show b, Eq b) => a -> b -> String



Составные типы.

Поля:
data User = User {
   firstName :: String,
   lastName  :: String,
   email     :: String
}
или кратко
data User = User {firstName, lastName, email :: String}


Это поля а заодно и функции геттеры!

user = User { firstName = "Denis"
  , lastName  = "Shevchenko"
  , email     = "me@dshevchenko.biz"
  }

firstName user выведет "Denis"


Можно делать и сеттеры, которые на самом деле возвращают копию объекта
с новым значением поля:

changeEmail :: User -> String -> User
changeEmail user newEmail = user { email = newEmail }


Конструктор типа - если мы сомнеываемся в типе каких либо полей. пример:
data User y = User { firstName::String, yearOfBird::y }

Теперь чтобы changeEmail работала с этим типом, нужно объявить её так:
changeEmail :: User String -> String -> User String
или полиморфно
changeEmail :: User a -> String -> User a



Наследуемые типы: их немного и они предопределены
можно наследоваться кортежем от нескольких.

Eq - эквивалентность == /=
Ord - сравниваемость < <= > >=
Enum - перечисляемость
Bounded - min max
Read,
Show

Пример:
data IPAddress = IP String deriving Show

Всё, IP теперь печатаемый. Способ его печати берется из использования, на лету:

main = print $ IP "127.0.0.1"
вывод
IP "127.0.0.1"




Собственные классы типов

type SHU = Integer  -- SHU (Scoville Heat Units), единица жгучести перца

class Pepper p where
	simple :: p -- константа, например
    color :: p -> String
    pungency :: p -> SHU

data Poblano = Poblano  -- Распространён в национальных блюдах Мексики.

data TrinidadScorpion = TrinidadScorpion  -- Самый жгучий перец в мире.

instance Pepper Poblano where
	simple Poblano "Test"
    color Poblano = "green"
    pungency Poblano = 1500

instance Pepper TrinidadScorpion where
	simple TrinidadScorpion "Test"
    color TrinidadScorpion = "red"
    pungency TrinidadScorpion = 855000


Что это нам даёт? Мы можем ограничить полиморфность.
допустим у нас есть класс Box, полиморфным полем которого
может быть ТОЛЬКО Pepper или Apple класс:

class (Pepper p, Apple p) => Box p where
    kind :: p -> String

Константу можно рассматривать как конструктор по умолчанию.




newtype - то же, что data, но нужен не для создания новых типов
а, скорее, обертки существующих типов. Используется для оптимизации
производительности
