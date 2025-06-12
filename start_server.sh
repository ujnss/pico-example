#!/bin/bash

curdir=$(pwd)

# zktls
cd ${curdir}/server
python3 https_server.py
