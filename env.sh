SUBMOD_SURFER_PATH="${BASH_SOURCE[0]%/*}"
submod-surfer() {
    cd "$("$SUBMOD_SURFER_PATH/target/release/submod_surfer" "$@")"
}
