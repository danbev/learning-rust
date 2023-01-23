## The Update Framework (TUF) client
This repository contains a very basic TUF client, the sole purpose of this is
document and understand how a TUF client works.

### Installing tuftool
This example is going to use a tool called
[tuftool](https://github.com/awslabs/tough/tree/develop/tuftool) to create the
example TUF repository that will be used by the example. It can be installed
using the following command:
```console
$ cargo install --force tuftool
```

And we can generate root.json, the private and public key used for
signing/verification using the following script:
```console
$ ./tuftool.sh
```

### Usage
```console
$ cargo r -- --repo-dir tuf_repo --trusted-root-json ./root.json --download-dir tuf_client
```
