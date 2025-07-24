#!/usr/bin/env python3

import os
import sys

print("Content-Type: text/plain\n")

path_info = os.environ.get("PATH_INFO", "")
print(f"CGI: Executing file: {sys.argv[1]}")
print(f"PATH_INFO: {path_info}")

body = sys.stdin.read()
print(f"Received Body:\n{body}")
