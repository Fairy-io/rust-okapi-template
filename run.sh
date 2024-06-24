#!/bin/bash

SCRIPT_NAME=$1



case "$SCRIPT_NAME" in
    dev)
        cargo watch --no-vcs-ignores -- ./scripts/dev.sh
        ;;

    test)
        cargo watch --no-vcs-ignores -- ./scripts/test.sh
        ;;
    
esac