from sqlalchemy import Column
from sqlalchemy import func
from sqlalchemy.sql.functions import Function


# TODO not sure about type Column[str|None]
def get_column_unicode(column: Column[str | None]) -> Function:
    result = column
    for old, new in _ACCENT_TO_NO_ACCENT_MAP.items():
        result = func.REPLACE(result, old, new)
    return result


def get_string_unicode(string: str) -> str:
    result = string
    for old, new in _ACCENT_TO_NO_ACCENT_MAP.items():
        result = result.replace(old, new)
    return result


_ACCENT_TO_NO_ACCENT_MAP = {
    "á": "a",
    "Á": "A",
    "é": "e",
    "É": "E",
    "í": "i",
    "Í": "I",
    "ó": "o",
    "Ó": "O",
    "ú": "u",
    "Ú": "U",
}
