
# Ping test

## Running

`cargo r -- www.baidu.com` or `cargo r --bin httping www.baidu.com`

## Help

```
Usage: httping [-k, --key] [-d, --debug] [-v, --verbose] [ARGS]

Options:
  -k, --key          Set the key of request ["token_20230313000136kwyktxb0tgspm00yo5"]
  -d, --debug        Enable debug mode
  -v, --verbose      Enable verbose mode

Args:
  host@1      The target url, for example: www.baidu.com

Create by araraloren v0.1.0
```