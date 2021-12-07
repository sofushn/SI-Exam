from clients.rest.rest_person import (
    create_person,
    delete_person,
    read_person,
    read_person_list,
    update_person,
    read_person_list_by_role
)
from clients.rest.rest_hybrid import (
    person_read_list_passed,
    person_read_list_passed_by_exam_name,
    person_read_list_failed,
    person_read_list_failed_by_exam_name
)
from entities.person import NewPerson
from flask import request


def person_route_create():
    try:
        create_person(NewPerson.from_json(request.json))
        return "200"
    except Exception as e:
        print(e)
        return "500"


def person_route_read_list():
    return read_person_list()


def person_route_update(id):
    try:
        update_person(id, NewPerson.from_json(request.json))
        return "200"
    except Exception as e:
        print(e)
        return "500"


def person_route_delete(id):
    try:
        delete_person(id)
        return "204"
    except Exception as e:
        print(e)
        return "500"


def person_route_read(id):
    try:
        return read_person(id)
    except Exception as e:
        print(e)
        return "500"


def person_route_read_list_passed():
    try:
        return person_read_list_passed()
    except Exception as e:
        print(e)
        return "500"


def person_route_read_list_passed_by_exam_name(name):
    try:
        return person_read_list_passed_by_exam_name(name)
    except Exception as e:
        print(e)
        return "500"


def person_route_read_list_failed():
    try:
        return person_read_list_failed()
    except Exception as e:
        print(e)
        return "500"


def person_route_read_list_failed_by_exam_name(name):
    try:
        return person_read_list_failed_by_exam_name(name)
    except Exception as e:
        print(e)
        return "500"
    
def person_route_read_list_by_role(role):
    try:
        return read_person_list_by_role(role)
    except Exception as e:
        print(e)
        return "500"
    


def collect_routes(app):
    app.add_url_rule("/person", view_func=person_route_create, methods=["POST"])
    app.add_url_rule("/person", view_func=person_route_read_list, methods=["GET"])
    app.add_url_rule(
        "/person/passed", view_func=person_route_read_list_passed, methods=["GET"]
    )
    app.add_url_rule(
        "/person/passed/<string:name>",
        view_func=person_route_read_list_passed_by_exam_name,
        methods=["GET"],
    )
    app.add_url_rule(
        "/person/failed", view_func=person_route_read_list_failed, methods=["GET"]
    )
    app.add_url_rule(
        "/person/failed/<string:name>",
        view_func=person_route_read_list_failed_by_exam_name,
        methods=["GET"],
    )
    app.add_url_rule(
        "/person/role/<string:role>",
        view_func=person_route_read_list_by_role,
        methods=["GET"],
    )
    app.add_url_rule("/person/<int:id>", view_func=person_route_update, methods=["PUT"])
    app.add_url_rule(
        "/person/<int:id>", view_func=person_route_delete, methods=["DELETE"]
    )
    app.add_url_rule("/person/<int:id>", view_func=person_route_read, methods=["GET"])
    
