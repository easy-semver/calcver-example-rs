# Calcver rust example


## `.calcver.yml`

Defaults are provided for most fields (see [calcver-cli/#12](https://github.com/easy-semver/calcver-cli/issues/12)). However, you have to specify at least one action to perform:

```yml
# -- uncomment to override defaults
#root: .
#repository_type: git
#prerelease_prefix: rc
#tag_regex: \d+\.\d+\.\d+
#major_regex: "BREAKING CHANGE:"
#minor_regex: ^feat
#patch_regex: ^fix

# specify cargo toml file
cargo: Cargo.toml

# OR a list of scripts

actions:
  - .\bin\build.sh

```

## Travis
To auto set version on build, setup a `.travis.yml` like this:

```yml
before_install:
  - cargo install calcver --bins

script: calcver && cargo build
```

1. Install the crate (including binaries)
1. Run calcver to bump version
1. Build/package/publish


