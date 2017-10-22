name:
    github-release edit -s $(cat .git-token) -u vmchale -r tin-summer -n "$(madlang run ~/programming/madlang/releases/releases.mad)" -t "$(grep -P -o '\d+\.\d+\.\d+' Cargo.toml | head -n1)"

check:
    git diff master origin/master

patch:
    cargo release -l patch --no-dev-version
