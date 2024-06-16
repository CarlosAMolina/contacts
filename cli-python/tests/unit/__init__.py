import pathlib
import sys

current_path = pathlib.Path(__file__).parent.absolute()
cli_python_path = current_path.parent.parent
cli_python_path_str = str(cli_python_path)
sys.path.append(cli_python_path_str)
