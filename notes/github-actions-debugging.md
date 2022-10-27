## Debugging github actions
It can be difficult to debug issues that only happen in github actions. 
I ran into and issue with Rust testcontainers which was running fine locally
but then failed in the github action flow:
https://github.com/seedwing-io/opa-client/pull/2

We can use [t-mate](https://dev.to/github/debug-your-github-actions-via-ssh-by-using-tmate-1hd6)
which allows us to get a ssh connection to the virtual machine and invesigate
what the error might be. 

To do this we have to update the github action in question:
```yaml
  - name: Setup tmate session
    uses: mxschmitt/action-tmate@v3
```
This can be done on the pull request branch and push it. If you look at the
logs of you will see it print out a ssh session url which we can use to
connect:
```console
ssh CMrrjndA5ZQ4B3XXr8NnCExXq@nyc1.tmate.io
```
This will give access to the environment environment and we can look around
and investigate the issue.

