# This script takes care of testing your crate

source env

set -ex

main() {
    #export RUST_BACKTRACE=1
    #cross build --target $TARGET --verbose
    just build

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    #cross test --lib --target $TARGET
    #cross run --target $TARGET -- check
    just test-all
    just run -- check

    ## sanity check the file type
    #file target/$TARGET/debug/art

    ## TODO: use rustfmt check fmt
    ## I want to do this but they KEEP CHANGING IT
    #cargo fmt -- --write-mode=diff

}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
