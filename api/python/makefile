run:
	export DB_URL=sqlite:////tmp/contacts.sqlite3 && python main.py

test:
	python -m unittest discover -s tests

test-filter:
	python -m unittest discover -s tests -k test_resolve_search_user_if_search_term_in_users_categories

