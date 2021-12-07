import requests
from utils.config import CONFIG
import json as JSON

_CLIENT_CONFIG: str = CONFIG["clients"]["rest"]["mini-proj"]
_PREFIX: str = f"http://{_CLIENT_CONFIG['host']}:{_CLIENT_CONFIG['port']}/exam"
_HEADERS = {"Content-Type": "application/json"}


def create_exam(exam):
    try:
        requests.post(_PREFIX, data=JSON.dumps(exam.__dict__), headers=_HEADERS)
    except Exception as e:
        print(e)


def read_exam(id):
    try:
        return requests.get(f"{_PREFIX}/{id}").json()
    except Exception as e:
        print(e)


def update_exam(id, exam):
    try:
        requests.put(
            f"{_PREFIX}/{id}", data=JSON.dumps(exam.__dict__), headers=_HEADERS
        )
    except Exception as e:
        print(e)


def delete_exam(id):
    try:
        return requests.delete(f"{_PREFIX}/{id}").text
    except Exception as e:
        print(e)


def read_list_exam():
    try:
        return JSON.dumps(requests.get(_PREFIX).json())
    except Exception as e:
        print(e)
