class SchoolClass:
    def __init__(self, id, subject, created_at, updated_at) -> None:
        self.id = id
        self.subject = subject
        self.created_at = created_at
        self.updated_at = updated_at

    @staticmethod
    def from_json(request):
        return SchoolClass(
            id=request.get("id"),
            subject=request.get("subject"),
            created_at=request.get("created_at"),
            updated_at=request.get("updated_at")
        )
