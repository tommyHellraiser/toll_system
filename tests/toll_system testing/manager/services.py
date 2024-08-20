import requests

from utilities.utilities import api_base_path, show_failed_test_message


def reset_database():
    url = api_base_path + "/manage/reset_schema"
    response = requests.put(url)

    if response.status_code != 200:
        show_failed_test_message(response, "Could not reset database")