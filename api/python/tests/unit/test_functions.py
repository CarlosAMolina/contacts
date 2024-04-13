import unittest


from src import db


class TestTODOrm(unittest.TestCase):
    def test_get_true(self):
        self.assertTrue(db.get_true())
