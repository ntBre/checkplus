#!/bin/bash

for c in {b,w}; do
    for p in {B,K,N,P,Q,R}; do
	curl -O 'https://raw.githubusercontent.com/lichess-org/lila/master/public/piece/cburnett/'"$c$p".svg
    done
done

