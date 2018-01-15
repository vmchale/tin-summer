# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    if [ "$TARGET" = "x86_64-unknown-redox" ]; then
        sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys AA12E97F0881517F
        sudo add-apt-repository 'deb https://static.redox-os.org/toolchain/apt /'
        sudo apt update
        sudo apt install x86-64-unknown-redox-gcc
        rustup target add x86_64-unknown-redox
        cargo build --bin sn --target $TARGET --release
    else
        cross rustc --bin sn --target $TARGET --release -- -C lto
    fi

    cp target/$TARGET/release/sn $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
