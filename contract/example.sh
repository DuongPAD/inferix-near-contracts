#!/bin/sh

echo ">> Set init contract"
near call jackyfinal.testnet init '{"governance": "jackyfinal.testnet"}' --accountId jackyfinal.testnet

# near view jackybook2.testnet get_all_donations '{}'