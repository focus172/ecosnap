from flask import Flask
import json


app = Flask(__name__)


@app.route("/search")
def search():
    file = open("data.json", "r")
    parsed = json.load(file)
    # name = input("Enter Brand:")
    print(parsed)
    file.close()
    return "<p>Hello, World!</p>"


def main():
    search()


if __name__ == "__main__":
    main()
