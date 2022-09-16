# Welcome to url-build-parse!
`url-build-parse` provides the ability to parse URL from string as well as construct URL from parts.

See [URL on Wikipedia](https://en.wikipedia.org/wiki/URL) and [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986) for more information.

Some supported URLs as an example (but not limited to):
 - ftp://ftp.is.co.za/rfc/rfc1808.txt
 - http://www.ietf.org/rfc/rfc2396.txt
 - ldap://[2001:db8::7]/c=GB?objectClass?one
 - mailto:John.Doe@example.com
 - news:comp.infosystems.www.servers.unix
 - tel:+1-816-555-1212
 - telnet://192.0.2.16:80/
 - urn:oasis:names:specification:docbook:dtd:xml:4.1.2


## Features
1. Convert given string into a UrlComponents struct
2. Convert given UrlComponents struct into a URL string



## Configuration
No additional configuration required.


## Demo

[Tests](https://github.com/bohdaq/url-build-parse/blob/main/src/lib.rs)
are available in the repository.

## Documentation
Public functions definitions and usage can be found at [git repository](https://github.com/bohdaq/url-build-parse/blob/main/src/lib.rs).


## Build
If you want to build `url-build-parse` on your own, make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

> $ cargo build


## Test
If you want to test `url-build-parse`.

> $ cargo test


## Community
Contact me on [Discord](https://discordapp.com/users/952173191659393025/) where you can ask questions and share ideas. Follow the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Donations
If you appreciate my work and want to support it, feel free to do it via [PayPal](https://www.paypal.com/donate/?hosted_button_id=7J69SYZWSP6HJ).
