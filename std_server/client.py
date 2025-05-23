#!/usr/bin/env python3
import requests
from typing import Dict
import json

URL: str = "http://localhost:3030"
HEADERS:Dict[str, str] = {
        "Content-Type": "application/json",
}

def hi() -> str | None:
    result = requests.get(url=URL, headers=HEADERS, stream=True)
    try:
        response = requests.get(
            url=URL, headers=HEADERS, stream=True)
        if response.status_code != 200:
            print(f"Error: {response.status_code}\n{response.text}")
            return None
        result = response.json()
        pretty = json.dumps(result, indent=4)
        print(pretty)
    except Exception as e:
        print(f"Unexpected error {e}")

hi()
