install:
    cargo install --features="simd pattern" --force

name:
    github-release edit -s $(cat .git-token) -u vmchale -r tin-summer -n "$(madlang run ~/programming/madlang/releases/releases.mad)" -t "$(grep -P -o '\d+\.\d+\.\d+' Cargo.toml | head -n1)"

check:
    git diff master origin/master

minor:
    cargo release -l minor --no-dev-version

patch:
    cargo release -l patch --no-dev-version
