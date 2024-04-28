import os
import unittest
from pathlib import Path


from src.db import database


class TestFunction_init_db(unittest.TestCase):
    def test_database_is_created(self):
        database.prepare_db()
        db_path_name = os.getenv("DB_PATH_NAME")
        self.assertIsNotNone(db_path_name)
        self.assertTrue(Path(db_path_name).is_file())
