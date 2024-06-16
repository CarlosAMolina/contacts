class TerminationId:
    ABNORMAL = 1
    SUCCESSFUL = 0


def run_iteractive():
    print("Welcome to the contacts CLI!")
    show_help()
    while True:
        print("Write an option or a search term and press enter")
        user_input = input()
        if user_input in ("exit", "q"):
            raise SystemExit(TerminationId.SUCCESSFUL)
        elif user_input in ("-i"):
            print("Start displaying ID")
            print("What ID do you want to see?")
            user_input = input()
            print("Retrieving ID", user_input)
        else:
            print("Searching term", user_input)


def show_help():
    print("Options:")
    print("- exit | q: exit the CLI")
    print("- -i: show a contact by ID")
    print("- Anything else you write, it will be the searched term")


if __name__ == "__main__":
    run_iteractive()
