from flask import Flask, request, jsonify
import json


app = Flask(__name__)

# loading the json file
with open ("data.json", 'r') as file:
    data = json.load(file)

@app.route("/search", methods=["POST"])
def search():
    data = request.json
    if data is None:
        return jsonify({"result": "Error missing request."})
    name = data["name"]
    if name is None:
        return jsonify({"result": "Error bad formated request."})

    file = open("data.json", "r")
    parsed = json.load(file)
    score = parsed[name]

    if score is None:
        return jsonify({"result": "Could not find thing thing."})
    # print(score)
    #
    file.close()
    return jsonify({"result": score})


@app.route("/getScores/<name>")
def getScores(name):

    if (name in data):
        return str(data[name])
        
    return f"Company '{name}' Not Found"

def main():
    search()


if __name__ == "__main__":
    main()
