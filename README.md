# cors-bypass-server

Lightweight musl binary for a performant http forward proxy

meant to be embedded into an application requiring use of an API that doesn't require CORS headers

## Usage

- `/?url=https://example.com`: the server makes a request to example.com, and returns the response.

## Configuration

- `CORS_PORT` env var: specifies which port to listen on, default is `9000`
- `WHITELIST` env var: comma separated list of whitelisted domains, defaults to accepting all domains
  - example: `WHITELIST=example.com,example.net,example.org`
- `CORS` env var: set if you want the server to add cors headers
