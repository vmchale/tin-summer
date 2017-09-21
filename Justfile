check:
    git diff master origin/master

patch:
    cargo release -l patch --no-dev-version
