import sys

try:
    from src.export import Export
    from src.search import IdSearch
    from src.search import TermSearch
except ModuleNotFoundError:
    from export import Export
    from search import IdSearch
    from search import TermSearch


class TerminationId:
    ABNORMAL = 1
    SUCCESSFUL = 0


def run():
    if len(sys.argv) == 1:
        run_interactive()
    elif sys.argv[1] == "-h":
        show_help()
    elif sys.argv[1] == "-e":
        Export().export_json()
    elif sys.argv[1] == "-i":
        IdSearch().run_search_value(sys.argv[2])
    else:
        TermSearch().run_search_value(sys.argv[1])


def run_interactive():
    print("Welcome to the contacts CLI!")
    while True:
        print("Write an option and press enter")
        show_help()
        user_input = input()
        if user_input in ("exit", "q"):
            raise SystemExit(TerminationId.SUCCESSFUL)
        else:
            if user_input == "-e":
                Export().export_json()
            elif user_input == "-i":
                IdSearch().run_ask_input()
            else:
                TermSearch().run_ask_input()


def show_help():
    print("Options:")
    print("- exit | q: exit the CLI")
    print("- -e: export db to JSON")
    print("- -i: search a contact by ID. All the contact information will be shown")
    print(
        "- No input, just press Enter: search for contacts by a search term. A summary of the information will appear"
    )


if __name__ == "__main__":
    run()
