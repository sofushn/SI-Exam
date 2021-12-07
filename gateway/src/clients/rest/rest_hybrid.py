import requests
from utils.config import CONFIG
import json as JSON

_CLIENT_CONFIG: str = CONFIG["clients"]["bus"]["mini-proj"]
_PREFIX: str = f"http://{_CLIENT_CONFIG['host']}:{_CLIENT_CONFIG['port']}/api/hybrid"

##### Post #####

##### Get #####


def person_read_list_passed():
    try:
        return JSON.dumps(requests.get(f"{_PREFIX}/passed").json())
    except Exception as e:
        print()


def person_read_list_passed_by_exam_name(exam_name):
    try:
        return JSON.dumps(requests.get(f"{_PREFIX}/passed/{exam_name}").json())
    except Exception as e:
        print()

def person_read_list_failed():
    try:
        return JSON.dumps(requests.get(f"{_PREFIX}/failed").json())
    except Exception as e:
        print()


def person_read_list_failed_by_exam_name(exam_name):
    try:
        return JSON.dumps(requests.get(f"{_PREFIX}/failed/{exam_name}").json())
    except Exception as e:
        print()


##### Put #####

##### Delete #####
