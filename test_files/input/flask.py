from flask import Flask

app = Flask(__name__)

@app.route('/')
def hello():
    return 'Hello, World!'

# post request
@app.route('/post', methods=['POST'])
def post():
    return 'POST'

if __name__ == '__main__':
    app.run()