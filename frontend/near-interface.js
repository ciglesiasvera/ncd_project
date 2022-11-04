/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

export class HelloNEAR {
    constructor({ contractId, walletToUse }) {
      this.contractId = contractId;
      this.wallet = walletToUse;    
    }
  
    async setPairKey() {
      return await this.wallet.callMethod({ contractId: this.contractId, method: 'create_update' });
    }
  
    async getPairKey(greeting) {
      return await this.wallet.viewMethod({ contractId: this.contractId, method: 'read', args: { k: String, v: String } });
    }

    async deletePairKey(greeting) {
        return await this.wallet.viewMethod({ contractId: this.contractId, method: 'delete', args: { k: String } });
      }
  }