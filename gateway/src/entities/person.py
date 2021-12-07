
class NewPerson():
    def __init__(self, first_name, last_name, phone_number, email, role) -> None:
        self.first_name = first_name
        self.last_name = last_name
        self.phone_number = phone_number
        self.email = email
        self.role = role
        
    @staticmethod
    def from_json(data):
        return NewPerson(
            first_name = data.get('first_name'),
            last_name = data.get('last_name'),
            phone_number= data.get('phone_number'),
            email = data.get('email'),
            role = data.get('role'),
        )
        
class Person():
    def __init__(self, first_name, last_name, phone_number, email, role, created_at, updated_at) -> None:
        self.first_name = first_name
        self.last_name = last_name
        self.phone_number = phone_number
        self.email = email
        self.role = role
        self.created_at = created_at
        self.updated_at = updated_at
        
    @staticmethod
    def from_grpc_response(response):
        return Person(
            first_name = response.first_name,
            last_name = response.last_name,
            phone_number= response.phone_number,
            email = response.email,
            role = response.role,
            updated_at = response.updated_at,
            created_at = response.created_at
        )
        
    @staticmethod
    def from_grpc_response_list(response):
        person_list = []
        for person in response.person_list:
            person_list.append(Person.from_grpc_response(person).__dict__)
        return person_list
        

    
