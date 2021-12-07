from typing import List
from requests.api import request
from entities.school_class import SchoolClass
import zeep
from utils.config import CONFIG

_CLIENT_CONFIG = CONFIG["clients"]["soap"]["mini-proj"]
_WSDL_URL = f"http://{_CLIENT_CONFIG['host']}:{_CLIENT_CONFIG['port']}/Service.svc?WSDL"


def _create_client():
    return zeep.Client(wsdl=_WSDL_URL)


def create_school_class(subject: str) -> int:
    return _create_client().service.CreateNewClass(subject)


def read_school_class(id: int) -> SchoolClass:
    response = _create_client().service.GetClass(id)
    schoolClass = SchoolClass(response.Id, response.Subject, response.CreatedAt, response.UpdatedAt)
    schoolClass.people = [person.Id for person in response.People.Person]
    return schoolClass

def read_list_school_class() -> list[SchoolClass]:
    return [
        SchoolClass(schoolClass.Id, schoolClass.Subject, schoolClass.CreatedAt, schoolClass.UpdatedAt) 
        for schoolClass in _create_client().service.GetAllClasses()
    ] 

def add_person(classId: int, personId: int) -> None:
    _create_client().service.AddPersonClass(classId, personId)

def delete_school_class(id: int) -> None:
    _create_client().service.RemoveClass(id)

def remove_person(id: int, personId: int) -> None:
    _create_client().service.RemovePerson(id, personId)