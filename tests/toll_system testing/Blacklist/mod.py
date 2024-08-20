import requests
from utilities.utilities import api_base_path


class Blacklist:
    def __init__(self, clients_id: None, license_plate, reason, restriction_expiry: None):

        self.id = None
        self.clients_id = clients_id
        self.license_plate = license_plate
        self.reason = reason
        self.restriction_expiry = restriction_expiry


    def build_body(self):
        body = {
            "license_plate": self.license_plate,
            "reason": self.reason
        }

        if self.clients_id is not None:
            body["clients_id"] = self.clients_id
        if self.restriction_expiry is not None:
            body["restriction_expiry"] = self.restriction_expiry

        return body

    def get_open_blacklist(self):
        url = api_base_path + "/blacklist"

        response = requests.get(url)
        return response


    def post_to_blacklist(self):
        url = api_base_path + "/blacklist"

        body = self.build_body()

        response = requests.post(url, json = body)
        return response

    def patch_blacklist_entry(self, reason=None, restriction_expiry=None):
        url = api_base_path + f"/blacklist/{self.id}"

        patch_body = {}

        if reason is not None:
            patch_body["reason"] = reason
        if restriction_expiry is not None:
            patch_body["restriction_expiry"] = restriction_expiry

        response = requests.patch(url, json=patch_body)

        return response
