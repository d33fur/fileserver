# Используем официальный образ Rust для сборки
FROM rust:1.60 AS build

# Устанавливаем рабочую директорию
WORKDIR /usr/src/app

# Копируем Cargo.toml и Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Копируем исходный код
COPY src ./src

# Сборка проекта
RUN cargo build --release

# Используем минимальный образ для запуска
FROM debian:buster-slim

# Копируем собранное приложение из предыдущего контейнера
COPY --from=build /usr/src/app/target/release/backend /usr/local/bin/backend

# Открываем порт 8001
EXPOSE 8001

# Запускаем приложение
CMD ["backend"]
