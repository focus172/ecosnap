from flask import Flask, request, jsonify
import json

# Init
app = Flask(__name__)
with open("data.json", "r") as file:
    data = json.load(file)


@app.route("/search", methods=["POST"])
def search():
    req = request.json
    if req is None:
        return jsonify({"error": "Error missing request."})

    name = req["name"]
    if name is None:
        return jsonify({"error": "Error bad formated request."})

    score = data[name]
    if score is None:
        return jsonify({"error": "Could not find thing thing."})

    return jsonify({"result": score})


# def main():
#     search()
#
# if __name__ == "__main__":
#     main()
