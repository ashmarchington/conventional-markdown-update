# Markdown Updater

This action updates a markdown file with the commits between current head and a specified before sha.

My workflow for personal projects uses conventional commit messages but not releases so this just updates
the changelog for each push.

> [!NOTE]
> This action can only be used in a Job that runs on a UNIX-like system currently (e.g `ubuntu-latest`)

## Basic Usage
The action script should be relatively self explanatory but included below is an example workflow, changelog and config file:

__You will need to set `contents: write` in permissions for the changelog to be recommited to the branch.__

```yaml
name: push_to_master

on:
  push:
    branches:
      - master

env:
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

jobs:
  update-changelog:
    name: update changelogs
    permissions:
      contents: write
      pull-requests: read

    runs-on: ubuntu-latest

    steps:
      - name: checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: update changelog
        id: changelog-update
        uses: ashmarchington/conventional-markdown-update@v1.0.0
        with:
          path: Changelog.md
          after: unreleased
          level: 2
          config: ./.github/conventional-markdown-update.toml
          base_ref: ${{ github.event.before }}
```

```toml
[[commit-rule]]
commit-type = "this-commit"
header-type = "maps-to-this"
```

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)

## unreleased

### maps-to-this
```

With a commit message of `this-commit: some commit` the changelog would be updated to;

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)

## unreleased

### maps-to-this
[sha](sha) `this-commit: some commit` **author** date
```
