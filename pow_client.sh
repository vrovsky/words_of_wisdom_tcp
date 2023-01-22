#!/bin/bash

IP="127.0.0.1"
PORT=5000
DIFFICULTY=4

echo "Starting PoW client"

while true; do
    # Generate a random nonce
    nonce=$RANDOM
    challenge="$IP$nonce"

    # Hash the challenge using SHA-256
    digest=$(echo -n $challenge | sha256sum | awk '{print $1}')

    # Check if the hash starts with enough zeroes
    if [[ $digest =~ ^0{$DIFFICULTY} ]]; then
        echo "PoW complete, sending request to server"
        # Connect to server and send the nonce as proof of work
        echo $nonce | nc $IP $PORT

        # Wait for server response
        response=$(nc -l $PORT)
        echo "Received: $response"
    else
        echo "PoW failed, trying again..."
    fi
done