FROM rust:1.82-alpine

WORKDIR /app
COPY . .

RUN apk add alpine-sdk
RUN cargo install --path .

CMD ["cargo", "run", "-r"]