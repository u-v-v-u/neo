#!usr/bin/env bash

if [[ -z "$1" ]]; then
    echo "You must specify a tag"
    exit
fi

git-cliff --tag "$1" > CHANGELOG.md
git add ./CHANGELOG.md
git commit -m "chore(release): $1"
git tag "$1"

echo "Finished! Please push the changes"