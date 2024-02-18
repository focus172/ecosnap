from typing import Any, List
from flask import Flask, Response, request, jsonify
import json
from google.cloud import vision
from difflib import SequenceMatcher
import base64

version = "0.1.0"

# Init
app = Flask(__name__)
with open("data.json", "r") as file:
    data = json.load(file)
client = vision.ImageAnnotatorClient()


@app.route("/get/<name>")
def get(name: str):
    return find([name])


@app.route("/search", methods=["POST"])
def search() -> Response:
    req = request.json
    if req is None:
        return error("Error missing request.")

    # Get the image data
    image_data = req["image"]
    content = base64.b64decode(image_data)

    # Get the logos in the photo
    names = get_logos(content)

    # get the response from the names
    return find(names)


def find(names: List[str]) -> Response:

    for guess in names:
        for company in data["main"]:
            accuracy_percentage = SequenceMatcher(None, guess.lower(), company.lower()).ratio()
            # if the guess is close to the company name, or if the company name is a word in the guess (separated from other words by spaces)
            # e.g. "Nike Shoes" should match "Nike" even though the actual strings aren't very close
            # e.g. the mispelling "buhlenciaga" should match Balenciaga but NOT "GU," another company in the lsit
            if (accuracy_percentage >= 0.75 or company.lower() in guess.lower().split()):
                name = company
                # print (f"{guess} matched {company} at {accuracy_percentage}")

    score = data["main"].get(name)
    if score is None:
        return error("Could not find anything with that name.")

    return jsonify(
        {
            "response": {
                "ok": score,
                "brand": name,
            },
            "version": version,
        }
    )


def error(desc: str) -> Response:
    return jsonify(
        {
            "response": {"err": desc},
            "version": version,
        }
    )


## Labels
# image = vision.Image(content=content)
# # Performs label detection on the image file
# response = client.label_detection(image=image)
# labels = response.label_annotations
# print("Labels:")
# for label in labels:
#     print(label.description)


## Detects logos in the file.
def get_logos(content: bytes) -> List[str]:
    image = vision.Image(content=content)

    response = client.logo_detection(image=image)

    if response.error.message:
        raise Exception(
            "{}\nFor more info on error messages, check: "
            "https://cloud.google.com/apis/design/errors".format(response.error.message)
        )

    logos = response.logo_annotations

    return [logo.description for logo in logos if True]
