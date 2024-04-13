import pathlib
import sys

current_path = pathlib.Path(__file__).parent.absolute()
api_python_path = current_path.parent.parent
api_python_path_str = str(api_python_path)
sys.path.append(api_python_path_str)
