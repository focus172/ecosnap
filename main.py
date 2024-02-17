from typing import AnyStr
from flask import Flask, request, jsonify
import json
import io
import os
from google.cloud import vision

# Init
app = Flask(__name__)
with open("data.json", "r") as file:
    data = json.load(file)
client = vision.ImageAnnotatorClient()


@app.route("/search", methods=["POST"])
def search():
    req = request.json
    if req is None:
        return jsonify({"error": "Error missing request."})

    name = req["name"]
    if name is None:
        return jsonify({"error": "Error bad formated request."})

    score = data["main"].get(name)
    if score is None:
        return jsonify({"error": "Could not find anything with that name."})

    return jsonify({"result": score})


# def main():
#     search()
#
# if __name__ == "__main__":
#     main()


def detect_labels(path: AnyStr):
    file = os.path.abspath(path)

    # The name of the image file to annotate

    # Loads the image into memory
    with io.open(file, "rb") as image:
        content = image.read()
    image = vision.Image(content=content)

    # Performs label detection on the image file
    response = client.label_detection(image=image)
    labels = response.label_annotations
    print("Labels:")
    for label in labels:
        print(label.description)


def detect_logos(path):
    """Detects logos in the file."""
    from google.cloud import vision

    client = vision.ImageAnnotatorClient()

    with open(path, "rb") as image_file:
        content = image_file.read()

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


# detect_labels("./test.png")
detect_logos("./nike-shoe.png")
