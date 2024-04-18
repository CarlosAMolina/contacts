import unittest
from pathlib import Path


from src.db import database


class TestFunction_init_db(unittest.TestCase):
    def test_database_is_created(self):
        database.prepare_db()
        self.assertTrue(Path(database._db_path_name).is_file())