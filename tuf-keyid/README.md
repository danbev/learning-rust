### tuf-keyid
This is a command line tool that takes a TUF public key in json format and
prints out the `key_id` for it. 

### Install
```console
$ cargo install --path .
```

Running:
```console
$ tuf-keyid --json='{
            "keytype": "rsa",
            "keyval": {
              "public": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA5ZiWzak3CBJkRrCfw5GO\nSUtYjIK2jLozyaZ44FePW/KYEhM8LyHcNz9lwx45tZ8gId4AsxGBj9fhsOgjpN7l\nMPXpaKsV/5f37HzQLCrbldz3ei9LkMWG5La4Cwil0qPDpTxfzI7IWDKk6l4/epgi\nOrAJDaQ/mKhH5OZ485JYuDIE7a0jplU/GvsNeCdZVMEQ8dko/CA4Di8lPkDRRdSw\naC/8g3K6mF+87ADdGOmZ+LFodLEPvqIVljece2JlX2z44Io3N7Y5FH63Az4H3MFL\nDPZJH5lFs7Lb/fHx25rWSE2/GHcUUTs4oScPp2X0hAnblOsmCFSCjf8Kb0R7dLUb\nnQIDAQAB\n-----END PUBLIC KEY-----"
            },
            "scheme": "rsassa-pss-sha256"
    }'
```

Running using cargo:
```console
$ cargo r -q -- --json='{
            "keytype": "rsa",
            "keyval": {
              "public": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA5ZiWzak3CBJkRrCfw5GO\nSUtYjIK2jLozyaZ44FePW/KYEhM8LyHcNz9lwx45tZ8gId4AsxGBj9fhsOgjpN7l\nMPXpaKsV/5f37HzQLCrbldz3ei9LkMWG5La4Cwil0qPDpTxfzI7IWDKk6l4/epgi\nOrAJDaQ/mKhH5OZ485JYuDIE7a0jplU/GvsNeCdZVMEQ8dko/CA4Di8lPkDRRdSw\naC/8g3K6mF+87ADdGOmZ+LFodLEPvqIVljece2JlX2z44Io3N7Y5FH63Az4H3MFL\nDPZJH5lFs7Lb/fHx25rWSE2/GHcUUTs4oScPp2X0hAnblOsmCFSCjf8Kb0R7dLUb\nnQIDAQAB\n-----END PUBLIC KEY-----"
            },
            "scheme": "rsassa-pss-sha256"
    }'
key_id: SHA256: 6e99ec437323f2c7334c8b16fd7a7a197829ba89ff50d07aa4b50fc9634dad9f
```
