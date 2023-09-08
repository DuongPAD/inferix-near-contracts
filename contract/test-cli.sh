#!/bin/sh

# echo ">> Set init contract"
# near call jackyfinal3.testnet init '{"governance": "jackyfinal3.testnet"}' --accountId jackyfinal3.testnet

# echo ">> Get contract status"
# near view jackyfinal.testnet contract_status '{}'

# echo ">> Get vault id"
# near view jackyfinal.testnet get_vault '{}'

# echo ">> Get allowance time"
# near view jackyfinal.testnet get_allowance_time '{}'

# echo ">> Call deposit"
# near call jackyfinal3.testnet deposit '{}' --accountId jackybook2.testnet --amount 6

# echo ">> Get deposited amount for account_id"
# near call jackyfinal3.testnet get_deposit_for_account '{"account_id": "jackybook2.testnet"}' --accountId jackybook.testnet

# echo ">> Get total number of users"
# near view jackyfinal.testnet get_number_of_users '{}'

# echo ">> Get total deposits"
# near view jackyfinal3.testnet get_deposits '{}'

# echo ">> Set spent for user"
# near call jackyfinal3.testnet set_user_spent '{"account_id": "jackybook2.testnet", "amount": 10000000000000}' --accountId jackyfinal3.testnet --amount 0.000000000000000000000001

# echo ">> Get spent for user"
# near call jackyfinal3.testnet get_user_spent '{"account_id": "jackybook2.testnet"}' --accountId jackybook.testnet

echo ">> withdraw_all"
near call jackyfinal3.testnet withdraw_all '{}' --accountId jackybook2.testnet --amount 0.000000000000000000000001
