#!/bin/bash

curdir=$(pwd)

# zktls
cd ${curdir}/server
python https_server.py
