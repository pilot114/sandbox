Haskell Platform - пакет для удобной работы с haskell.

ghc/ghci - компилятор/интерпретатор haskell
cabal - утилита для сбоки пакетов
stack - кроссплатформенная утилита для управления проектами

Пример проекта:

Real
└── src
    ├── Main.hs # main module
    └── Utils
        └── Helpers.hs # import module

use cabal:
cabal init - генерация проекта
cabal configure - настройка
cabal build - собираем


Удобнее всего использовать Стак - он установит хаскелл и все зависимости.

stack setup
stack new test
# or for simple project template: stack new test simple
cd test
stack build
# run
stack exec real-exe
# or move exe to PATH: stack install && real-exe

Этого достаточно чтобы писать на haskell =)



проект - пакеты - модули

Модули - это haskell файлы, один модуль = один файл.
У каждого модуля нужно указывать имя.
В конфиге проекта нужно перечислять все модули и пути, где их искать.

Пакет - это набор модулей
Hackage - гигантский репозиторий haskell пакетов
Для поиска пакетов можно использовать тот же cabal:

cabal update
cabal install text

Только надо не забыть обновить конфиг проекта - и можно юзать =)
Prelude - модуль, загружаемый по умолчанию.
