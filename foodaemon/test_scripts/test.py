#!/bin/python

import socket;  # noqa: I001

soc = socket.socket(
    family = socket.AF_UNIX,
    type = socket.SOCK_STREAM,
)

soc.connect("/run/user/1000/wayland-1-foodaemon");
