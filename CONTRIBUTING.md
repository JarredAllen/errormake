Contributing
============

## Suggesting a Change

If you have a change you'd like to see made to this project, create an issue in
the Github repository. In the issue, please be as detailed as possible in
explaining what you would like to see changed.

### Bug Reports

If you are reporting a bug, please include the simplest possible case that
demonstrates the buggy behavior, and an explanation of what it does.

### Feature Requests

If you are requesting a feature, please include a snippet of code which
demonstrates how to use the feature and, if possible, a reason why it should be
included.

### Documentation Changes

If you are requesting a change to the documentation, describe why the current
documentation is insufficient (Is it unclear? Is it incomplete? Is it
outdated?). If you can also suggest changes to fix the issue, that will make
implementing the change much quicker.

## Implementing Suggested Changes

If there's a change suggested that no one is currently working on implementing,
and you want to work on it reply to the issue indicating your desire to work on
it, and then implement the changes.

To make changes, fork this repository, make changes on your own personal branch,
and then create a pull request to add your code to the repository.

### Code Style

To maintain a consistent style, this project uses rustfmt. To format your code
to the desired style, run `cargo fmt`.

### Testing

This project uses Travis CI to automatically run tests in a uniform environment.
It should automatically run on any pull request you create and display its
results in the pull request feed.

In testing, Travis CI runs the following sequence of commands:
```
cargo build
cargo test --verbose
cargo fmt -- --check
cargo clippy -- -D warnings
```
You can run those commands locally on your machine to verify that all tests
pass.
