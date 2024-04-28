import os
import pathlib
import sys

current_path = pathlib.Path(__file__).parent.absolute()
api_python_path = current_path.parent.parent
api_python_path_str = str(api_python_path)
sys.path.append(api_python_path_str)
_DB_PATH_NAME = "/tmp/contacts.sqlite3"
_DB_URL = f"sqlite:///{_DB_PATH_NAME}"
os.environ["DB_PATH_NAME"] = _DB_PATH_NAME
os.environ["DB_URL"] = _DB_URL
