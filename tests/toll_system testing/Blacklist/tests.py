from Blacklist.mod import Blacklist
from utilities.utilities import show_test_title_function, show_failed_test_message
import json
from datetime import datetime, timedelta

def run_blacklist_tests():

    test_01_get_open_blacklist()
    test_02_create_new_blacklist()
    test_03_create_new_and_patch()

def test_01_get_open_blacklist():
    show_test_title_function("TEST 01 - Get open Blacklist")

    blacklist = Blacklist(None, "AB123AB", "He's a dumbass", None)
    response = blacklist.get_open_blacklist()

    if response.status_code != 200:
        show_failed_test_message(response)
    if len(json.loads(response.content)) != 1:
        show_failed_test_message(response, "Failed to compare body length")

    print("Ok.")

def test_02_create_new_blacklist():
    show_test_title_function("TEST 02 - Create new Blacklist")

    blacklist = Blacklist(None, "AB123AB", "He's a dumbass", None)
    response = blacklist.post_to_blacklist()

    if response.status_code != 200:
        show_failed_test_message(response)

    print("Ok.")

def test_03_create_new_and_patch():
    show_test_title_function("TEST 03 - Create new and patch")

    blacklist = Blacklist(None, "AS456DS", "Just because", None)
    response = blacklist.post_to_blacklist()

    if response.status_code != 200:
        show_failed_test_message(response, "Failed to create a Blacklist entry")

    blacklist.id = json.loads(response.content)["id"]

    expiry = (datetime.now() + timedelta(days=1)).strftime('%Y-%m-%dT%H:%M:%S')
    response = blacklist.patch_blacklist_entry("New reason to Blacklist", expiry)

    if response.status_code != 200:
        show_failed_test_message(response, "Could not update Blacklist entry")

    print("Ok.")
