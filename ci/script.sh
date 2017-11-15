# This script takes care of testing your crate

set -ex

main() {
    cross build --target $TARGET --features $FEATURES
    cross build --target $TARGET --features $FEATURES --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET --features $FEATURES
    cross test --target $TARGET --features $FEATURES --release

    cross run --target $TARGET --features $FEATURES -- a
    cross run --target $TARGET --features $FEATURES --release -- a
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
