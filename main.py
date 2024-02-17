from flask import Flask, request
import json


app = Flask(__name__)

# loading the json file
with open ("data.json", 'r') as file:
    data = json.load(file)

@app.route("/search")
def search():
    name = request.json
    print(name)
    file = open("data.json", "r")
    parsed = json.load(file)
    # name = input("Enter Brand:")
    print(parsed)
    file.close()
    return "<p>Hello, World!</p>"


@app.route("/getScores/<name>")
def getScores(name):

    if (name in data):
        return str(data[name])
        
    return f"Company '{name}' Not Found"

def main():
    search()


if __name__ == "__main__":
    main()
