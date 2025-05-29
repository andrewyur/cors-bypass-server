# cors-bypass-server

Lightweight musl binary for a performant http forward proxy

meant to be embedded into an application requiring use of an API that doesn't require CORS headers

## Usage

- `/?url=https://example.com`: the server makes a request to example.com, and returns the response.

## Configuration

- `PORT` env var: specifies which port to listen on, default is `8080`
- `WHITELIST` env var: comma separated list of whitelisted domains, defaults to accepting all domains
  - example: `WHITELIST=example.com,example.net,example.org`
