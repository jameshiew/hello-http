#!/bin/bash

set -eoux pipefail

curl --fail http://${HTTP_HOST}:${HTTP_PORT}/livez || exit 1