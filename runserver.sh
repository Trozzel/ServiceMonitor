#!/bin/bash

WATCH_DIR=${PWD}/frontend
STATUS_FILE=${PWD}/.dir_status
STATUS_FILE_OLD=${PWD}/.dir_status_old

# First time build
if [ ! -e ${STATUS_FILE} ]; then
    ls -la ${WATCH_DIR} --time-style=full-iso > ${STATUS_FILE}
fi

# If not any changes, skip transpile process
mv ${STATUS_FILE} ${STATUS_FILE_OLD}
ls -la ${WATCH_DIR} --time-style=full-iso > ${STATUS_FILE}
DIFF=`diff ${STATUS_FILE} ${STATUS_FILE_OLD}`
rm ${STATUS_FILE_OLD}

if [[ ! -z "$DIFF" ]]; then
    cd ${WATCH_DIR}
    npm run build
    ls -la --time-style=full-iso > ${STATUS_FILE}
    cd -
fi

# RUN ROCKET SERVER
cargo run --bin runserver
