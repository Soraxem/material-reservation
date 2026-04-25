FROM rust

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV DATABASE_URL=postgres://user:thePassword@localhost/inventory

EXPOSE 8080

ADD . /app
WORKDIR /app

RUN cargo install cargo-watch
CMD ["cargo", "watch", "-x", "run"]