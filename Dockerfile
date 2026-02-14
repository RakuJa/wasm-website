# Stage 1: Build
FROM rust:1-alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    wget

RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

# Build the project using trunk
RUN trunk build --release

# Stage 2: Deploy
FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]