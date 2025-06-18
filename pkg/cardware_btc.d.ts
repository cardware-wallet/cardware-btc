/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} psbt_bytes
* @returns {(string)[]}
*/
export function convert_psbt_to_qr(psbt_bytes: Uint8Array): (string)[];
/**
* @param {string} transaction
* @param {string} esplora_url
* @returns {Promise<string>}
*/
export function raw_broadcast(transaction: string, esplora_url: string): Promise<string>;
/**
*/
export class Wallet {
  free(): void;
/**
* @param {(string)[]} xpubs
* @param {string} esplora_url
* @param {string} network
*/
  constructor(xpubs: (string)[], esplora_url: string, network: string);
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
* @param {(string)[]} transaction_signatures
* @returns {Promise<string>}
*/
  broadcast_multisig(transaction_signatures: (string)[]): Promise<string>;
/**
* @param {string} derivation
* @returns {Promise<string>}
*/
  get_tx_history(derivation: string): Promise<string>;
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
* @param {number} number_of_blocks
* @returns {bigint}
*/
  estimate_sweep_fee(number_of_blocks: number): bigint;
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
