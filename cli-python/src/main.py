try:
    from src.search import IdSearch
    from src.search import TermSearch
except ModuleNotFoundError:
    from search import IdSearch
    from search import TermSearch


class TerminationId:
    ABNORMAL = 1
    SUCCESSFUL = 0


def run_interactive():
    print("Welcome to the contacts CLI!")
    show_help()
    while True:
        print("Write an option or a search term and press enter")
        user_input = input()
        if user_input in ("exit", "q"):
            raise SystemExit(TerminationId.SUCCESSFUL)
        elif user_input == "-i":
            IdSearch().run()
        else:
            TermSearch().run(user_input)


def show_help():
    print("Options:")
    print("- exit | q: exit the CLI")
    print("- -i: show a contact by ID")
    print("- Anything else you write, it will be the searched term")


if __name__ == "__main__":
    run_interactive()
