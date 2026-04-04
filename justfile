set dotenv-load := true

default:
    @just --list

run-backend:
    -podman rm -f catalog-backend
    podman run --rm \
    --name catalog-backend \
    -e CARGO_TERM_COLOR=always \
    -v "$(pwd)/backend:/workspace:Z" \
    -v cargo-target:/workspace/target \
    -v cargo-registry-cache:/usr/local/cargo/registry \
    -e DB_TOKEN=$DB_TOKEN \
    -e DB_URL=$DB_URL \
    -e CLIENT_URL=$CLIENT_URL \
    -e SECRET_KEY=$SECRET_KEY \
    -p 8080:80 \
    -w /workspace \
    rust:alpine \
    sh -c "apk add --no-cache musl-dev openssl-dev pkgconfig gcc && cargo run"

run-frontend:
    -podman rm -f catalog-frontend
    podman run --rm -it --init \
      --name catalog-frontend \
      -e VITE_API_URL=$API_URL \
      -e VITE_STORE_NAME="$STORE_NAME" \
      -v ./frontend:/app:Z \
      -w /app \
      -p "5173:5173" \
      denoland/deno:debian \
      deno run dev --host
    

build-container:
  #!/usr/bin/env bash
  set -euo pipefail
 
  podman build -t "ferlis:dev" -f - . <<EOF
  FROM alpine:latest
  RUN apk add --no-cache ca-certificates tzdata
    
  COPY backend/target/debug/backend /usr/local/bin/backend
    
  EXPOSE 80
  CMD ["backend"]
  EOF


open-vs-frontend:
  code frontend
enter: open-vs-frontend 
  zellij --layout dev.kdl
