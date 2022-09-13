FROM rust:1.63.0

WORKDIR /usr/src/app

COPY . .
RUN cargo build --locked --release --package prisma-cli

RUN cargo prisma migrate dev
RUN cargo prisma generate

RUN cargo build --release --locked
CMD ["target/release/core"]