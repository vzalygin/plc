# Postfix language compiler

Компилятор для стекового языка с [постфиксным синтаксисом](https://ru.wikipedia.org/wiki/%D0%9E%D0%B1%D1%80%D0%B0%D1%82%D0%BD%D0%B0%D1%8F_%D0%BF%D0%BE%D0%BB%D1%8C%D1%81%D0%BA%D0%B0%D1%8F_%D0%B7%D0%B0%D0%BF%D0%B8%D1%81%D1%8C).

Компилирует для платформы `linux-x86_64`, для работы нужно установить `nasm`.

- [X] арифметические операции
- [ ] dup, drop, take
- [ ] биндинги
- [ ] ветвление
- [ ] функции

## Поддерживаемый синтаксис

- 32-битные числа со знаком;
- Арифметические операторы `+`, `-`, `*`, `/`;
- Оператор вывода в stdout `.`;
- Комментарии, начинающиеся с `#` до конца строки.

## Как использовать

В системе должны быть установлены `cargo`, `nasm`. Компиляция осуществляется только для `linux-x86_64`.

Скомпилировать компилятор:

```bash
git clone https://github.com/vzalygin/plc
cd plc
cargo build --release --manifest-path ./compiler/Cargo.toml
sudo cp ./compiler/target/release/plc /usr/bin
```

Для информации об интерфейсе:

```bash
plc -h
```

## Примеры

Доступные в папке [examples](./examples)
