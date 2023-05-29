 
# Гра  камінь, ножиці, папір:


# Залежності
Для роботи програми потрібні наступні бібліотеки

```
[dependencies]
rand = { version = "0.8.5", features = ["small_rng"] }
owo-colors = "3.4.0"
```

# Компіляція
Виконайте наступні команди для компіляції

~~~bash  
  git clone https://github.com/faunel/game.git
  cd game
  cargo build --release
~~~

Потім перейдіть в каталог програми
~~~bash 
  cd target/release
~~~

# Використання

- Вводьте в консоль цифру від 1 до 3

- Комп'ютер теж в свою чергу введе випадкове число

- Виграш відбувається відповідно правил

<img src="https://github.com/faunel/game/blob/master/img/rules.png" width="500">

- [Аккаунт](https://htmlpreview.github.io/?https://github.com/andrey-tech/amocrm-api-v2-docs/blob/master/docs/account.html) 