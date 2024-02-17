from typing import AnyStr
from flask import Flask, Response, request, jsonify
import json
from google.cloud import vision

version = "0.1.0"

# Init
app = Flask(__name__)
with open("data.json", "r") as file:
    data = json.load(file)
client = vision.ImageAnnotatorClient()


@app.route("/get/<name>")
def get(name: AnyStr):
    return find(name)


@app.route("/search", methods=["POST"])
def search() -> Response:
    req = request.json
    if req is None:
        return error("Error missing request.")

    name = req["name"]
    if name is None:
        return error("Error bad formated request.")

    return find(name)


def find(name: AnyStr) -> Response:
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


def error(desc: AnyStr) -> Response:
    return jsonify(
        {
            "response": {"err": desc},
            "version": version,
        }
    )


# def detect_labels(path: AnyStr):
#     file = os.path.abspath(path)
#
#     # The name of the image file to annotate
#
#     # Loads the image into memory
#     with io.open(file, "rb") as image:
#         content = image.read()
#     image = vision.Image(content=content)
#
#     # Performs label detection on the image file
#     response = client.label_detection(image=image)
#     labels = response.label_annotations
#     print("Labels:")
#     for label in labels:
#         print(label.description)


## Detects logos in the file.
def detect_logos(content: bytes):
    image = vision.Image(content=content)

    response = client.logo_detection(image=image)
    logos = response.logo_annotations
    print("Logos:")

    for logo in logos:
        print(logo.description)

    if response.error.message:
        raise Exception(
            "{}\nFor more info on error messages, check: "
            "https://cloud.google.com/apis/design/errors".format(response.error.message)
        )


with open("./nike-shoe.png", "rb") as image_file:
    content = image_file.read()

# detect_labels("./test.png")
detect_logos(content)
