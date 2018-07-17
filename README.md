# esa-nippou

`esa-nippou` is print today's your [esa.io](https://esa.io) articles.

`esa-nippou` is inspired by [masutaka/github-nippou](https://github.com/masutaka/github-nippou).

## Setup

Please execute `esa-nippou init` command. It Sets required configurations on interactive mode.

```console
$ esa-nippou init
Personal access token (hidden):
Team: example
Screen name: mizukmb
$ cat ~/.esanippourc
esanippou:
  team: example
  screen_name: mizukmb
  parsonal_access_token: XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

or enviroment variables (load priority: .esanippourc < environment variables).

- `ESA_NIPPOU_ACCESS_TOKEN` (**required**)
  - esa.io personal access tokens
  - You can generate this token from https://[team].esa.io/user/applications
- `ESA_NIPPOU_TEAM` (**required**)
  - Name of the team you belong to
- `ESA_NIPPOU_USERNAME` (**required**)
  - Your screen name (NOT full name)

## Build

```console
$ cargo build
```

or download from [release page](https://github.com/mizukmb/esa-nippou/releases) (`x86_64-apple-darwin` only).

## Usage

```console
$ esa-nippou
# print today's your articles...
```

## Example

```console
$ esa-nippou
- [path/to/mizukmb の日報](https://example.esa.io/posts/123456) by @mizukmb
```

## Author

[@mizukmb](https://twitter.com/mizukmb)

## License

MIT
