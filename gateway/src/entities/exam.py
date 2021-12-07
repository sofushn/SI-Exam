class Exam:
    def __init__(self, name, examination_date) -> None:
        self.name = name
        self.examination_date = examination_date

    @staticmethod
    def from_json(data):
        return Exam(
            name=data.get("name"), examination_date=data.get("examination_date")
        )
