/* tslint:disable */
/* eslint-disable */
/**
*/
export class Wallet {
  free(): void;
/**
* @param {string} xpub
* @param {string} esplora_url
* @param {string} fingerprint
* @param {string} network
*/
  constructor(xpub: string, esplora_url: string, fingerprint: string, network: string);
/**
* @returns {Promise<string>}
*/
  sync(): Promise<string>;
/**
* @param {string} max_depth
* @returns {Promise<string>}
*/
  sync_to_depth(max_depth: string): Promise<string>;
/**
* @param {string} transaction
* @returns {Promise<string>}
*/
  broadcast(transaction: string): Promise<string>;
/**
* @param {(string)[]} recipient_addrs
* @param {BigUint64Array} amounts
* @param {bigint} fee
* @returns {(string)[]}
*/
  send(recipient_addrs: (string)[], amounts: BigUint64Array, fee: bigint): (string)[];
/**
* @param {(string)[]} recipient_addrs
* @param {BigUint64Array} amounts
* @param {number} number_of_blocks
* @returns {bigint}
*/
  estimate_fee(recipient_addrs: (string)[], amounts: BigUint64Array, number_of_blocks: number): bigint;
/**
* @param {(string)[]} utxo_vec
*/
  set_trusted_pending(utxo_vec: (string)[]): void;
/**
* @returns {(string)[]}
*/
  get_trusted_pending(): (string)[];
/**
* @returns {bigint}
*/
  balance(): bigint;
/**
* @returns {bigint}
*/
  unconfirmed_balance(): bigint;
/**
* @returns {string}
*/
  address(): string;
/**
* @param {string} derivation_path
* @returns {string}
*/
  new_address(derivation_path: string): string;
}
