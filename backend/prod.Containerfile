FROM rust:alpine AS builder
# Declare required envs
ENV CARGO_TERM_COLOR=always

# User 
RUN adduser -H -D -u 10001 appuser

# install deps
RUN apk add --no-cache musl-dev pkgconfig gcc perl make

# copy source code
COPY . /workspace
WORKDIR /workspace

# compile
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/workspace/target \
    cargo build --release && \
    cp /workspace/target/release/backend /workspace/final_binary
# create runner container

FROM scratch

# copy users
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# copy certificates
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ 

# copy binary
COPY --from=builder /workspace/final_binary /app

# runner env varibles
ENV DB_TOKEN=""
ENV DB_URL=""
ENV CLIENT_URL=""
ENV SECRET_KEY=""
ENV PORT=8080

USER appuser

ENTRYPOINT ["/app"]
