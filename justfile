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
    -e PORT=8080 \
    -e RUST_LOG=debug \
    -p 8080:8080 \
    -w /workspace \
    rust:alpine \
    sh -c "apk add --no-cache musl-dev pkgconfig gcc perl make && cargo run"

build-backend:
    podman build \
    -f prod.Containerfile \
    -t catalog:latest \
    ./backend

run-backend-prod:
    -podman rm -f catalog-backend-prod
    podman run --rm \
    --name catalog-backend-prod \
    -e DB_TOKEN=$DB_TOKEN \
    -e DB_URL=$DB_URL \
    -e CLIENT_URL=$CLIENT_URL \
    -e SECRET_KEY=$SECRET_KEY \
    -e PORT=8080 \
    -p 8080:8080 \
    localhost/catalog

upload-backend-image:
  podman tag localhost/catalog docker.io/haterofvectors/catalog:latest
  podman push docker.io/haterofvectors/catalog:latest


run-frontend:
    -podman rm -f catalog-frontend
    podman run --rm -it --init \
      --name catalog-frontend \
      -e VITE_API_URL=$API_URL \
      -e VITE_STORE_NAME="$STORE_NAME" \
      -e VITE_STORE_PHONE_NUMBER="$STORE_PHONE_NUMBER" \
      -v ./frontend:/app:Z \
      -w /app \
      -p "5173:5173" \
      denoland/deno:debian \
      deno run dev --host

build-frontend:
    podman run --rm -it --init \
      --name catalog-frontend-builder \
      -e VITE_API_URL=$PROD_API_URL \
      -e VITE_STORE_NAME="$STORE_NAME" \
      -e VITE_STORE_PHONE_NUMBER="$STORE_PHONE_NUMBER" \
      -v ./frontend:/app:Z \
      -w /app \
      denoland/deno:debian \
      deno run build
    

open-vs-frontend:
  code frontend
enter: open-vs-frontend 
  zellij --layout dev.kdl
