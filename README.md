# EcoSnap
Hack for Humanity 2024 Project

## Dependencies
### V1
```bash
pip install -r api/v1/requirements.txt
```
### V2
A working rust tool chain.

## Running
### V1
Requires a valid API key located at `./api/v1/google-cloud-vision-key.json`
```bash
just api
```
### V1
Requires `gcloud` installed on your system and you to be logged in:
```bash
just api-v2
```
