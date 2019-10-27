#!/bin/bash
set -e -o pipefail

# We are leaving this script in place so that old guides / documentation will
# point users to the new ways of installing alt. This is a much better
# alternative than having users get a 404 and passing that to bash so that it
# gets evaluated.

echo '👋 Hi there!'
echo 'alt no longer supports this installation method'
echo 'For alternate installation instructions, please see:'
echo '  https://github.com/dotboris/alt#install'

exit 1
