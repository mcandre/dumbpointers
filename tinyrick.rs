//! Build configuration

extern crate tinyrick;
extern crate tinyrick_extras;

/// Generate documentation
fn doc() {
    tinyrick_extras::build();
}

/// Run clippy
fn clippy() {
    tinyrick_extras::clippy();
}

/// Validate documentation and run linters
fn lint() {
    tinyrick::deps(doc);
    tinyrick::deps(clippy);
}

/// Lint and run tests
fn test() {
    tinyrick::deps(lint);
    tinyrick_extras::unit_test();
}

/// BUILD: Doc, lint, test, compile project
fn build() {
    tinyrick::deps(test);
    tinyrick_extras::build();
}

/// Publish to crate repository
fn publish() {
    tinyrick_extras::publish();
}

/// Clean workspaces
fn clean() {
    tinyrick_extras::clean_cargo();
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(clean);

    tinyrick::wubba_lubba_dub_dub!(
        build;
        doc,
        clippy,
        lint,
        test,
        publish,
        clean
    );
}
