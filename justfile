default:
  @just -l

run:
  GOOGLE_APPLICATION_CREDENTIALS=google-cloud-vision-key.json python -m flask --app main run

post name:
  curl --header "Content-Type: application/json" \
    --request POST \
    --data '{"name":"{{name}}"}' \
    http://localhost:5000/search
