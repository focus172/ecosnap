
alias api := api-v1

default:
    @just -l

# Runs the V1 server
api-v1:
    cd api/v1 && GOOGLE_APPLICATION_CREDENTIALS=google-cloud-vision-key.json python main.py

# Runs the V2 server
api-v2:
    cd api/v2 && cargo run

# Runs the app for local testing
app:
    cd app && npx expo

build:
    just build-ext
    just build-api-v2

build-ext:
    cp ./assets/icon.png ./ext/chrome/images

build-api-v2:
    cd api/v2 && cargo build
