test:
    cargo test --all-features -- -q --nocapture

update-changelog version:
    git cliff --tag {{ version }}
    git add "CHANGELOG.md"
    git commit -m "updated changelog"

release-test version: test
    cargo release {{ version }} -p protoschema

release-exec version: test (update-changelog version)
    cargo release {{ version }} -p protoschema --execute
