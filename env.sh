# ZSH doesn't like BASH_SOURCE, and git bash doesn't like $0
# We can add more checks here as issues with other shells arise
SHELL_FILE=$(echo "$SHELL" | sed "s/.*\///")
echo $SHELL_FILE
if [ "$SHELL_FILE" = "zsh" ]; then
    SUBMOD_SURFER_PATH="${0%/*}"
else 
    SUBMOD_SURFER_PATH="${BASH_SOURCE[0]%/*}"
fi
echo $SUBMOD_SURFER_PATH

submod-surfer() {
    cd "$("$SUBMOD_SURFER_PATH/target/release/submod_surfer" "$@")"
}
