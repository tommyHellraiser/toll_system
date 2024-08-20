from Blacklist.tests import run_blacklist_tests
from manager.services import reset_database

# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    reset_database()

    run_blacklist_tests()
