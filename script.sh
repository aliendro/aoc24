#!/bin/sh

for i in $(seq 1 25); do
	if [[ $i -ge 0 && $i -le 9 ]]; then
		filename="0$i"
	else
		filename="$i"
	fi
	aoc download -d $i -i data/inputs/$filename -p data/puzzles/$filename.md --session-file data/adventofcode.session
done
