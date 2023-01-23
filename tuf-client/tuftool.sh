#!/bin/bash

tuftool root init root.json

tuftool root set-threshold root.json snapshot 1
tuftool root set-threshold root.json root 1
tuftool root set-threshold root.json timestamp 1
tuftool root set-threshold root.json targets 1

tuftool root expire root.json 'in 6 weeks'

tuftool root gen-rsa-key root.json ./keys/root.pem --role root

tuftool root add-key root.json ./keys/root.pem --role targets
tuftool root add-key root.json ./keys/root.pem --role timestamp
tuftool root add-key root.json ./keys/root.pem --role snapshot

tuftool root sign root.json -k ./keys/root.pem 

tuftool create \
  --root root.json \
  --key keys/root.pem \
  --add-targets artifacts \
  --targets-expires 'in 3 weeks' \
  --targets-version 1 \
  --snapshot-expires 'in 3 weeks' \
  --snapshot-version 1 \
  --timestamp-expires 'in 1 week' \
  --timestamp-version 1 \
  --outdir tuf_repo

cp root.json tuf_client
