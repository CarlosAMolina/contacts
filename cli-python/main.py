import sys


class TerminationId:
    ABNORMAL = 1
    SUCCESSFUL = 0


def get_what_to_run_parsing_args():
    if (args_count := len(sys.argv)) == 1:
        run_iteractive()
    elif args_count == 2:
        search_term = sys.argv[1]
        print("Searching term", search_term)
    elif args_count == 3:
        option = sys.argv[1]
        if option != "-i":
            print("Invalid option", option)
            raise SystemExit(TerminationId.ABNORMAL)
        else:
            search_id = sys.argv[2]
            print("Start displaying ID", search_id)
    else:
        print("Unmanaged situation")
        raise SystemExit(TerminationId.ABNORMAL)


def run_iteractive():
    print("Starting iteractive mode")
    print("Welcome to the contacts CLI!")
    print("- Write `exit` or `q` to exit the CLI")
    while True:
        print("What do you want to search?")
        search_term = input()
        if search_term in ("exit", "q"):
            raise SystemExit(TerminationId.SUCCESSFUL)
        else:
            print("Searching term", search_term)


if __name__ == "__main__":
    get_what_to_run_parsing_args()
