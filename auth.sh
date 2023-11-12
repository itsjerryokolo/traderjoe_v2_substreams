#!/bin/bash

# Request the token
response=$(curl -s https://auth.streamingfast.io/v1/auth/issue --data-binary '{"api_key":"server_5e8c3b24e3dc1d12e73c10b0ad1e5556"}')

# Extract the token from the response
token=$(echo $response | jq -r .token)

# Print the token
echo "Token: $token"

# Export the token as an environment variable
export SUBSTREAMS_API_TOKEN=$token
echo "Token has been set."
