host := "172.31.164.78:6699"

default:
  @just -l

run:
  GOOGLE_APPLICATION_CREDENTIALS=google-cloud-vision-key.json python main.py

get name:
  curl http://{{host}}/get/{{name}}

post path:
  printf '{"image":"' > .tmp.file
  base64 -w 0 "{{path}}" >> .tmp.file
  echo '"}' >> .tmp.file

  curl http://{{host}}/search \
    --header "Content-Type: application/json" \
    --request POST \
    --data @.tmp.file

  rm .tmp.file

deploy:
  gcloud run deploy --source .
