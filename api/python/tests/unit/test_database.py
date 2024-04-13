import unittest
from pathlib import Path


from src import database


class TestFunction_init_db(unittest.TestCase):
    def test_database_is_created(self):
        database.init_db()
        self.assertTrue(Path(database._db_path_name).is_file())
