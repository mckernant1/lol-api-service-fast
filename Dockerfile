FROM 653528873951.dkr.ecr.us-west-2.amazonaws.com/rust:alpine-arm AS build 

RUN apk add protoc pkgconfig openssl-dev musl-dev libc6-compat

RUN mkdir /app
COPY . /app

WORKDIR /app

RUN cargo build --release


FROM 653528873951.dkr.ecr.us-west-2.amazonaws.com/rust:alpine-arm AS run

RUN apk add protoc pkgconfig openssl-dev musl-dev libc6-compat

RUN mkdir /app

COPY --from=build /app/target/release/lol-api-service-fast /app/lol-api-service-fast

ENTRYPOINT /app/lol-api-service-fast
