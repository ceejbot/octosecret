# octosecret

Encrypt secrets using libsodium sealed boxes, via [sodiumoxide](https://docs.rs/sodiumoxide/0.2.5/sodiumoxide/). My use case is to set [Github secrets](https://developer.github.com/v3/actions/secrets/#create-or-update-a-secret-for-a-repository) in some automated workflows; you might find it similarly handy.

Takes one argument: a base64-encoded public key. Reads the secret value from stdin. It sends to stdout the base64-encoded result of encrypting the input with the public key.

```shell
echo "it's a secret to everyone" | octosecret <b64-encoded-public-key> <<<
```

If you have administrative access to a github repo plus an api token handy, you can fetch its public key like this:

```shell
http GET https://api.github.com/repos/ceejbot/octosecret/actions/secrets/public-key Authorization:"token deadbeef" | jq .key
```

## Installing

Downloadable binaries for darwin & amd64 are provided on the releases page.

## License

MIT
