#!/usr/bin/env bash

cd "$(dirname "$0")/.."

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" >$OUTDIR/hello1.txt
echo "Hello" "there" >$OUTDIR/hello2.txt
echo -n "Hello  there" >$OUTDIR/hello1.n.txt
echo -n "Hello" "there" >$OUTDIR/hello2.n.txt
