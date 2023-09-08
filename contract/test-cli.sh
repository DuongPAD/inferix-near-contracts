#!/bin/sh

# echo ">> Set init contract"
# near call inferix.testnet init '{"governance": "inferix.testnet"}' --accountId inferix.testnet

# echo ">> Get contract status"
# near view inferix.testnet contract_status '{}'

# echo ">> Get vault id"
# near view inferix.testnet get_vault '{}'

# echo ">> Get allowance time"
# near view inferix.testnet get_allowance_time '{}'

echo ">> Call deposit"
near call inferix.testnet deposit '{}' --accountId jackybook2.testnet --amount 6

# echo ">> Get deposited amount for account_id"
# near call inferix.testnet get_deposit_for_account '{"account_id": "jackybook2.testnet"}' --accountId jackybook.testnet

# echo ">> Get total number of users"
# near view inferix.testnet get_number_of_users '{}'

# echo ">> Get total deposits"
# near view inferix.testnet get_deposits '{}'

# echo ">> Set spent for user"
# near call inferix.testnet set_user_spent '{"account_id": "jackybook2.testnet", "amount": 10000000000000}' --accountId inferix.testnet --amount 0.000000000000000000000001

# echo ">> Get spent for user"
# near call inferix.testnet get_user_spent '{"account_id": "jackybook2.testnet"}' --accountId jackybook.testnet

# echo ">> withdraw_all"
# near call inferix.testnet withdraw_all '{}' --accountId jackybook2.testnet --amount 0.000000000000000000000001
