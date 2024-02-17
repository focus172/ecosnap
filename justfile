default:
  @just -l

run:
  python -m flask --app main run

post name:
  curl --header "Content-Type: application/json" \
    --request POST \
    --data '{"name":"{{name}}"}' \
    http://localhost:5000/search
