ACCENT_TO_NO_ACCENT_MAP = {
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


def get_string_unicode(string: str) -> str:
    result = string
    for old, new in ACCENT_TO_NO_ACCENT_MAP.items():
        result = result.replace(old, new)
    return result
