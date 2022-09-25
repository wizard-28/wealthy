# Contribution guidelines

First off, thank you for considering contributing to wealthy.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/wizard-28/wealthy/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

### Committing

Follow the [Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0) specification for your
commit messages.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/wizard-28/wealthy/blob/master/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.1.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

First fork the project, and clone your fork to your machine.

You'll need to install [Just](https://github.com/casey/just#installationt)
next. Think of it like the `make` command.

After you've installed Just, run the following command to setup your
development environment:

```shell
just # or `just setup-dev`
```

That will install the following development tools to your machine:

* Nightly `rustfmt` for formatting the code in the project.
* `pre-commit` for pre-commit hooks, which run before you commit to make sure
  that your commit is up to standards.
* `codespell` for spell checking.

Then run the tests to verify that your development environment is setup correctly.

```shell
just test
```

### Useful Commands

- List all Just recipes:

  ```shell
  just -l
  ```

- Build the project

  ```shell
  just build
  ```

- Run all checks (`spellcheck`, `clippy`, `test`, `fmt`):

  ```shell
  just check
  ```

  - Run the spell checker:

    ```shell
    just spellcheck
    ```

  - Run Clippy:

    ```shell
    just clippy
    ```

  - Run all tests:

    ```shell
    just test
    ```

  - Format the code in the project

    ```shell
    just fmt
    ```
