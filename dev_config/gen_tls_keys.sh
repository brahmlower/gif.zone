#!/usr/bin/env bash

mkdir -p ./keys
rm -f ./keys/*
openssl req -new -x509 -config test_keys.conf -nodes -out ./keys/test.gif.zone.crt
chmod 400 ./keys/test.gif.zone.key
