# This script takes care of testing your crate

set -ex

main() {
    if [ "$TARGET" = "x86_64-unknown-redox" ]; then
        sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys AA12E97F0881517F
        sudo add-apt-repository 'deb https://static.redox-os.org/toolchain/apt /'
        sudo apt update
        sudo apt install x86-64-unknown-redox-gcc
        rustup target add x86_64-unknown-redox
        cargo build --target "$TARGET" --release
    else
        cross build --target "$TARGET"
        cross build --target "$TARGET" --release

        if [ ! -z "$DISABLE_TESTS" ]; then
            return
        fi

        cross test --target "$TARGET"
        cross test --target "$TARGET" --release

        cross run --target "$TARGET" -- a
        cross run --target "$TARGET" --release -- a
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z "$TRAVIS_TAG" ]; then
    main
fi
