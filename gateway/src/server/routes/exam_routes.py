from clients.rest.rest_exam import (
    create_exam,
    read_exam,
    update_exam,
    delete_exam,
    read_list_exam,
)
from entities.exam import Exam
from flask import request

def exam_route_create():
    try:
        create_exam(Exam.from_json(request.json))
        return "200"
    except Exception as e:
        print(e)
        return "500"


def exam_route_read_list():
    return read_list_exam()


def exam_route_update(id):
    try:
        update_exam(id, Exam.from_json(request.json))
        return "200"
    except Exception as e:
        print(e)
        return "500"


def exam_route_delete(id):
    delete_exam(id)
    return "204"


def exam_route_read(id):
    return read_exam(id)


def collect_routes(app):
    app.add_url_rule("/exam", view_func=exam_route_create, methods=["POST"])
    app.add_url_rule("/exam", view_func=exam_route_read_list, methods=["GET"])
    app.add_url_rule("/exam/<int:id>", view_func=exam_route_update, methods=["PUT"])
    app.add_url_rule("/exam/<int:id>", view_func=exam_route_delete, methods=["DELETE"])
    app.add_url_rule("/exam/<int:id>", view_func=exam_route_read, methods=["GET"])
