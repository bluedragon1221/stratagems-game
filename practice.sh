if ! command -v fzf >/dev/null; then
  echo "fzf is not installed. Try running
'nix-shell -p fzf'
in the shell to continue"
  exit 1
fi


STRATAGEMS=(
  "500 KG Bomb"
  "Reinforce"
  "Quasar Cannon"
  "Resupply"
  "Expendable Anti-Tank"
  "MG-43 Machine Gun"
  "Gattling Century"
 )

COMMAND="cargo run --"

printf 'How many to practice: '
read -r count
COMMAND+=" -c $count"

MODE=$(printf "randomize\nselect" | fzf)

case "$MODE" in
  randomize) ;;
  select)
      COMMAND+=$(printf '%s\n' "${STRATAGEMS[@]}" | \
      fzf --multi | \
      xargs -d '\n' printf ' -p "%s"')
  ;;
esac

printf '%s\n' "$COMMAND"
