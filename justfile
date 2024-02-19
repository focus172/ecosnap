# host := "172.31.164.78:6699"
host := "localhost:6699"

default:
  @just -l

# Runs the V1 server
run:
  GOOGLE_APPLICATION_CREDENTIALS=google-cloud-vision-key.json python main.py

# Runs the app for local testing
app:
  npx expo

# Makes a GET request to the server to look up name
get name:
  curl http://{{host}}/v2/get/{{name}}

# Makes a GET request to the server to match name
find name:
  curl http://{{host}}/v2/find/{{name}}

# Makes a post request to the server
post path:
  printf '{"data":"' > .tmp.file
  base64 -w 0 "{{path}}" >> .tmp.file
  echo '"}' >> .tmp.file

  curl http://{{host}}/v2/search \
    --header "Content-Type: application/json" \
    --request POST \
    --data @.tmp.file

  rm .tmp.file

# Deploy the could to google cloud
deploy:
  gcloud run deploy --source .
