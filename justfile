default:
  @just -l

run:
  GOOGLE_APPLICATION_CREDENTIALS=google-cloud-vision-key.json python -m flask --app main run

post path:
  printf '{"image":"' > .tmp.file
  base64 -w 0 "{{path}}" >> .tmp.file
  echo '"}' >> .tmp.file

  curl http://localhost:5000/search \
    --header "Content-Type: application/json" \
    --request POST \
    --data @.tmp.file

  rm .tmp.file
