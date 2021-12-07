from clients.rest.rest_grade import (
    grade_create,
    grade_read_list,
    grade_read_list_by_exam_id,
    grade_read_list_by_person_id,
    grade_update_by_exam_id,
    grade_update_by_person_id,
    grade_delete_by_exam_id,
    grade_delete_by_person_id,
)

from entities.grade import Grade


def grade_route_create():
    try:
        grade_create(Grade.from_json(request.json))
        return "200"
    except Exception as e:
        print(e)
        return "500"


def grade_route_read_list():
    return grade_read_list()


def grade_route_read_list_by_exam_id(id):
    return grade_read_list_by_exam_id(id)


def grade_route_read_list_by_person_id(id):
    return grade_read_list_by_person_id(id)


def collect_routes(app):
    app.add_url_rule("/grade", view_func=grade_route_create, methods=["POST"])

    app.add_url_rule("/grade", view_func=grade_route_read_list, methods=["GET"])
    app.add_url_rule(
        "/grade/exam/<int:id>",
        view_func=grade_route_read_list_by_exam_id,
        methods=["GET"],
    )
    app.add_url_rule(
        "/grade/person/<int:id>",
        view_func=grade_route_read_list_by_person_id,
        methods=["GET"],
    )