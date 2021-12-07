from typing import ValuesView
from entities.school_class import SchoolClass
from clients.soap.soap_school_class import (
    create_school_class,
    read_school_class,
    read_list_school_class,
    add_person,
    delete_school_class,
    remove_person
)
from flask import request
import json


def school_class_route_create():
    return create_school_class(request.json.get("subject"))


def school_class_route_read_list():
    classes = read_list_school_class()
    return json.dumps([schoolClass.__dict__ for schoolClass in classes], default=str)


def school_class_add_person(id: int):
    personId = request.json.get("person_id")
    add_person(id, personId)

def school_class_route_read(id):
    schoolClass = read_school_class(id)
    return json.dumps(schoolClass.__dict__, default=str)

def school_class_route_delete(id: int):
    delete_school_class(id)

def school_class_route_remove_person(id: int, person_id: int):
    remove_person(id, person_id)

def collect_routes(app):
    app.add_url_rule(
        "/class", view_func=school_class_route_create, methods=["POST"]
    )
    app.add_url_rule(
        "/class", view_func=school_class_route_read_list, methods=["GET"]
    )
    app.add_url_rule(
        "/class/<int:id>/person", view_func=school_class_add_person, methods=["POST"]
    )
    app.add_url_rule(
        "/class/<int:id>", view_func=school_class_route_read, methods=["GET"]
    )
    app.add_url_rule(
        "/class/<int:id>", view_func=school_class_route_delete, methods=["DELETE"]
    )
    app.add_url_rule(
        "/class/<int:id>/person/<int:person_id>", view_func=school_class_route_remove_person, methods=["DELETE"]
    )
