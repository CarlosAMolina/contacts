try:
    from src.search import IdSearch
    from src.search import TermSearch
except ModuleNotFoundError:
    from search import IdSearch
    from search import TermSearch


class TerminationId:
    ABNORMAL = 1
    SUCCESSFUL = 0


def run():
    run_interactive()


def run_interactive():
    print("Welcome to the contacts CLI!")
    while True:
        print("Write an option and press enter")
        show_help()
        user_input = input()
        if user_input in ("exit", "q"):
            raise SystemExit(TerminationId.SUCCESSFUL)
        else:
            if user_input == "-i":
                IdSearch().run()
            else:
                TermSearch().run()


def show_help():
    print("Options:")
    print("- exit | q: exit the CLI")
    print("- -i: search a contact by ID. All the contact information will be shown")
    print(
        "- No input, just press Enter: search for contacts by a search term. A summary of the information will appear"
    )


if __name__ == "__main__":
    run()
