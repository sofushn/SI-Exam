import requests
from utils.config import CONFIG
import json as JSON

_CLIENT_CONFIG: str = CONFIG["clients"]["bus"]["mini-proj"]
_PREFIX: str = f"http://{_CLIENT_CONFIG['host']}:{_CLIENT_CONFIG['port']}/api/person"
_HEADERS = {"Content-Type": "application/json"}


def create_person(new_person):
    try:
        requests.post(_PREFIX, data=JSON.dumps(new_person.__dict__), headers=_HEADERS)
    except Exception as e:
        print(e)


def read_person(id):
    try:
        return requests.get(f"{_PREFIX}/{id}").json()
    except Exception as e:
        print(e)


def update_person(id, new_person):
    try:
        requests.put(
            f"{_PREFIX}/{id}", data=JSON.dumps(new_person.__dict__), headers=_HEADERS
        )
    except Exception as e:
        print(e)


def delete_person(id):
    try:
        return requests.delete(f"{_PREFIX}/{id}").text
    except Exception as e:
        print(e)


def read_person_list():
    try:
        return JSON.dumps(requests.get(_PREFIX).json())
    except Exception as e:
        print(e)

def read_person_list_by_role(role):
    try:
        print(requests.get(f"{_PREFIX}/role/{role}").json())
        return JSON.dumps(requests.get(f"{_PREFIX}/role/{role}").json())
    except Exception as e:
        print(e)