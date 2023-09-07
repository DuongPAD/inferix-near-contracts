/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

import { utils } from 'near-api-js'

export class Contract {

  constructor({ contractId, walletToUse }) {
    this.contractId = contractId;
    this.wallet = walletToUse;
  }

  async getBeneficiary() {
    return await this.wallet.viewMethod({ contractId: this.contractId, method: "get_beneficiary" })
  }

  async latestdeposits() {
    const number_of_donors = await this.wallet.viewMethod({ contractId: this.contractId, method: "number_of_users" })
    const min = number_of_donors > 10 ? number_of_donors - 9 : 0

    let deposits = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_deposits", args: { from_index: min.toString(), limit: number_of_donors } })

    deposits.forEach(elem => {
      elem.total_amount = utils.format.formatNearAmount(elem.total_amount);
    })

    return deposits
  }

  async getdepositFromTransaction(txhash) {
    let deposit_amount = await this.wallet.getTransactionResult(txhash);
    return utils.format.formatNearAmount(deposit_amount);
  }

  async deposit(amount) {
    let deposit = utils.format.parseNearAmount(amount.toString())
    let response = await this.wallet.callMethod({ contractId: this.contractId, method: "deposit", deposit })
    return response
  }

}