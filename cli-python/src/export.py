from search import AllSearch


class Export:
    def export_json(self):
        print("Exporting JSON")
        db_json = AllSearch().get_all_db()
        json = _Json(db_json)
        self._assert_all_ids_exist(json.get_ids())
        #print(db_json)  # TODO rm

    def _assert_all_ids_exist(self, ids: list[int]):
        expected_value = 1
        for id_ in ids:
            if not id_ == expected_value:
                raise ValueError(f"Lost ID: {id_}")
            expected_value += 1


class _Json:
    def __init__(self, json: dict):
        self._json = json

    def get_ids(self) -> list[int]:
        return sorted([user["id"] for user in self._get_users()])

    def _get_users(self) -> list[dict]:
        return self._json["data"]["usersWithTerm"]

