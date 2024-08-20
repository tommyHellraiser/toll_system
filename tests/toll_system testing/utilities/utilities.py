api_base_path = "http://127.0.0.1:8090/api"


def show_test_title_function(test_title):
    print(f"\n\n--- RUNNING {test_title} ---")

def show_failed_test_message(response, error_message=None):

    print(f"\n\nSome tests failed to run...")
    if error_message is not None:
        print(error_message)
    print(response.status_code)
    print(response.content)

    exit(-1)