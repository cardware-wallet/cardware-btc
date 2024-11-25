# Cardware BTC NPM Library

This is the documentation for the BTC NPM package that communicates with the Cardware device.

It allows users to create a watch-only wallet on the web.

All data that is transferred between the web wallet and the Cardware device is done through scanning QR codes.

Users must first pair the their web wallet with their Cardware device.

Once paired they are then able to view the address of their Cardware device, see their confirmed and uncofirmed balances and also send from their device.

When sending, the watch only wallet will create an unsigned transaction which will be split up into QR codes. The user will then be prompted to scan these QRcodes with their Cardware device. The user will then confirm the transactions details which will then create a signed transaction which their Cardware device will split up into QR codes. The web wallet then scans these QR codes, decodes them and then broadcasts the transaction.

---

# Documentation

## Initialization

### Code

```javascript
import Wallet from 'cardware-btc'; 
```

---

## New Wallet

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| xpub | string | The xpub of the the hardware wallet. | "vpub5ZNhc5KKM6hACK6QDuo6UG1749XUeXf9Gbu8rcZQnNDeMJwUPrwzEVKsF7X7EzZe5yqwymfMA1tGJ9qAmjdmGHSkRW7SruCEDz9mgEkwWvN" |
| esplora_url | string | The address of the esplora you are using. | "https://blockstream.info/testnet/api" |
| fingerprint | string | The fingerprint used for identifying the correct xpub. | "fa436c5b" |
| network | string | The network you are using (mainnet or testnet). | "mainnet" |

### Code

```javascript
var wallet = await new Wallet(xpub, esplora_url, fingerprint, network);
```

### Output

No outputs.

---

## Sync

### Parameters

No parameters.

### Code

```javascript
await wallet.sync();
```

### Output

The output is a string.

| Result | Description | Output |
|---|---|---|
| success | The wallet has synced successfully. | ```"Sync successful."``` |
| error | There is an issue fetching the fee estimates. | ```"Error: Fee sync error."``` |
| error | There is an issue with connecting to the esplora url. | ```"Error: Failed to connect to full node (Esplora)."``` |
| error | There is an issue deserializing the transaction. | ```"Error: Failed to deserialize transaction."``` |

---

## Sync to Depth

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| max_depth | string | The max depth of the addresses you need to sync. | "m/0/0" |

### Code

```javascript
await wallet.sync_to_depth(max_depth);
```

### Output

The output is a string.

| Result | Description | Output |
|---|---|---|
| success | The wallet has synced successfully. | ```"Sync successful."``` |
| error | There is an issue fetching the fee estimates. | ```"Error: Fee sync error."``` |
| error | There is an issue with connecting to the esplora url. | ```"Error: Failed to connect to full node (Esplora)."``` |
| error | There is an issue deserializing the transaction. | ```"Error: Failed to deserialize transaction."``` |

---

## Fees

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| addresses | array[string] | The addresses to send to. | ["tb1qvdl9rvg3m5ghfnppw2728rd92059pfqe0a8jjv"] |
| amounts | array[int64] | The send amounts in satoshis. | [4500] |
| number_of_blocks | int32 | The number of blocks for fee estimation. The lower the number, the higher the fee. | 3 |


### Code

```javascript
let fee = wallet.estimate_fee(addresses, amounts, number_of_blocks);
```

### Output

The output is a uint64.

| Result | Description | Output |
|---|---|---|
| success | The fee estimation for a transaction.. | ```1120``` |
| error | The addresses array and the amounts array are not the same length. | ```0``` |
| error | There is an issue parsing the network. | ```1``` |
| error | There is an invalid recipient address. | ```2``` |
| error | There is an issue fetching the fee estimates.. | ```3``` |
| error | There is insufficient BTC to make this transaction. | ```4``` |
| error | There are no UTXOs to spend. | ```5``` |

---

## Send

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| addresses | array[string] | The addresses to send to. | ["tb1qvdl9rvg3m5ghfnppw2728rd92059pfqe0a8jjv"] |
| amounts | array[int64] | The send amounts in satoshis. | [4500] |
| fee | int64 | The transaction fee worked out in estimate_fee. | 1120 |


### Code

```javascript
var qrcode_chunks = wallet.send(addresses, amounts, fee);
```

### Output

The output is an array of strings.

| Result | Description | Output |
|---|---|---|
| success | An array of base64 strings which can be shown as QR codes. | ```[, , , ]``` |
| error | The addresses array and the amounts array are not the same length. | ```["Error: Recipients and amounts arrays must be the same length."]```
| error | The address is not associated with the BTC network. | ```["Error: Failed to parse network."]```
| error | There is insufficient BTC to make this transaction. | ```["Error: Insufficient funds."]```
| error | There are no UTXOs to spend. | ```["Error: No UTXOs to spend."]```
| error | There is an issue with the derivation path. | ```["Error: Derivation path error."]```
| error | There is an invalid recipient address. | ```["Error: Invalid recipient address."]```

---

## Broadcast

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| signed_transaction | string | The signed transaction that needs to be broadcasted | TODO |

### Code

```javascript
await wallet.broadcast(signed_transaction);
```

### Output

The output is a strings.

| Result | Description | Output |
|---|---|---|
| success | An array of base64 strings which can be shown as QR codes. | ```""``` |
| error | There is an issue parsing the base64 transaction string. | ```"Error: Failed to parse base64 transaction."```
| error | There is an issue decoding the hex transaction.| ```"Error: Decoding failed."```
| error | The signed transaction is invalid. | ```"Error: Invalid transaction."```
| error | There is an issue broadcasting the transaction. | ```"Error: Failed to broadcast transaction."```

---

## Address

### Parameters

No parameters.

### Code

```javascript
const result = wallet.address();
```

### Output

The output is a string.

| Result | Description | Output |
|---|---|---|
| success | The address of the wallet. | ```"tb1qvdl9rvg3m5ghfnppw2728rd92059pfqe0a8jjv"``` |
| error | There is an invalid extended public key. | ```"Error: Invalid extended public key."``` |
| error | There is an issue deriving the xpub. | ```"Error: Xpub derivation error."``` |

---

## New Address

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| derivation_path | string | The derivation path the new address must be derived from. | "m/0/0" |

### Code

```javascript
const result = wallet.new_address(derivation_path);
```

### Output

| Result | Description | Output |
|---|---|---|
| success | The address of the wallet. | ```"tb1qvdl9rvg3m5ghfnppw2728rd92059pfqe0a8jjv"``` |
| error | There is an invalid extended public key. | ```"Error: Invalid extended public key."``` |
| error | There is an issue deriving the xpub. | ```"Error: Xpub derivation error."``` |

---

## Balance

### Parameters

No parameters.

### Code

```javascript
const result = wallet.balance();
```

### Output

The output is a unint64.

| Result | Description | Output |
|---|---|---|
| success | The confirmed balance of your wallet.| ```"69520"``` |
---

## Unconfirmed Balance

### Parameters

No parameters.

### Code

```javascript
const result = wallet.unconfirmed_balance();
```

### Output

The output is a unint64.

| Result | Description | Output |
|---|---|---|
| success | The unconfirmed balance of your wallet.| ```"4562"``` |

---