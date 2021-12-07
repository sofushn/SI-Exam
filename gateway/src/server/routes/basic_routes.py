def home_route():
    return (
        "This is the home of the gateway. There is not meant to be anything useful here"
    )


def health_check_route():
    return "OK"


def collect_routes(app):
    app.add_url_rule("/", view_func=home_route, methods=["GET"])
    app.add_url_rule("/health", view_func=health_check_route, methods=["GET"])
