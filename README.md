# esa-nippou

`esa-nippou` is print today's your [esa.io](https://esa.io) articles.

`esa-nippou` is inspired by [masutaka/github-nippou](https://github.com/masutaka/github-nippou).

## Setup

Please set enviroment variables.

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

or download from release page (WIP).

## Usage

```console
$ esa-nippou
# print today's your articles...
```

## Example

Set up enviroment variables.

```console
$ export ESA_NIPPOU_ACCESS_TOKEN=XXXXX...
$ export ESA_NIPPOU_TEAM=example
$ export ESA_NIPPOU_USERNAME=mizukmb
```

Execute.

```console
$ esa-nippou
- [path/to/mizukmb の日報](https://XXXXXX.esa.io/posts/123456) by @mizukmb
```

## Author

[@mizukmb](https://twitter.com/mizukmb)

## License

MIT
