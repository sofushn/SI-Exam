import logging
from utils.config import CONFIG
from server.routes import exam_routes
from flask import Flask


from server.routes import exam_routes
from server.routes import school_class_routes
from server.routes import grade_routes
from server.routes import person_routes
from server.routes import basic_routes

app = Flask(__name__)

basic_routes.collect_routes(app)
exam_routes.collect_routes(app)
school_class_routes.collect_routes(app)
grade_routes.collect_routes(app)
person_routes.collect_routes(app)


def main() -> None:
    app.run(debug=True, host=CONFIG["server"]["host"], port=CONFIG["server"]["port"])


if __name__ == "__main__":
    main()
