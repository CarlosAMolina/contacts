from search import AllSearch


class Export:
    def export_json(self):
        print("Exporting JSON")
        db_json = AllSearch().get_all_db()
        print(db_json)  # TODO rm
