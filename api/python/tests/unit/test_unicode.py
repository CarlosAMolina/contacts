import unittest

from sqlalchemy.sql.functions import Function
from sqlalchemy import Column

from src.utils import unicode

STRING_NO_UNICODE_VALUES = "ÁÉÍÓÚáéíóú"
STRING_UNICODE_VALUES = "AEIOUaeiou"


class TestFunction_get_column_unicode(unittest.TestCase):
    def test_no_unicode_values_are_replaced(self):
        value_to_modify = Column(STRING_NO_UNICODE_VALUES)
        result = unicode.get_column_unicode(value_to_modify)
        self.assertIsInstance(result, Function)
        # TODO create a test that really checks the value is converted

    def test_none_db_value_does_not_raise_exception(self):
        value_to_modify = Column(None)
        result = unicode.get_column_unicode(value_to_modify)
        self.assertIsInstance(result, Function)
        # TODO create a test that really checks the value is converted


class TestFunction_get_string_unicode(unittest.TestCase):
    def test_no_unicode_values_are_replaced(self):
        result = unicode.get_string_unicode(STRING_NO_UNICODE_VALUES)
        self.assertEqual(STRING_UNICODE_VALUES, result)

    def test_no_exception_if_input_is_an_empty_string(self):
        self.assertEquals("", unicode.get_string_unicode(""))
