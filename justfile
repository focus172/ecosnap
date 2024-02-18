default:
  @just -l

run:
  GOOGLE_APPLICATION_CREDENTIALS=google-cloud-vision-key.json python -m flask --app main run

get name:
  curl http://172.31.164.78:6699/get/{{name}}
  # --header "Content-Type: application/json" \
  # --request POST \
  # --data @.tmp.file

post path:
  printf '{"image":"' > .tmp.file
  base64 -w 0 "{{path}}" >> .tmp.file
  echo '"}' >> .tmp.file

  curl http://172.31.164.78:6699/search \
    --header "Content-Type: application/json" \
    --request POST \
    --data @.tmp.file

  rm .tmp.file

req path:
  printf '{"image":"' > .tmp.file
  base64 -w 0 "{{path}}" >> .tmp.file
  echo '"}' >> .tmp.file

  curl http://ecosnap-ddkjfurzhq-uw.a.run.app/search \
    --header "Content-Type: application/json" \
    --request POST \
    --data @.tmp.file

  rm .tmp.file

deploy:
  gcloud run deploy --source .
