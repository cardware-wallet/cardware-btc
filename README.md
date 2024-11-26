# Cardware BTC NPM Library

This is the documentation for the BTC NPM package that communicates with the Cardware device.

The library requires an electrs endpoint to query the bitcoin blockchain.

It allows users to create a watch-only wallet on the web.

All data that is transferred between the web wallet and the Cardware device is done through scanning QR codes.

Users must first pair the web wallet with their Cardware device.

Once paired they are then able to view the BTC address of their Cardware device, see their confirmed and unconfirmed BTC balances and send BTC from their Cardware device.

When sending, the watch only wallet will create an unsigned transaction which will be split up into QR codes. The user will then be prompted to scan these QRcodes with their Cardware device. The user will then confirm the transactions details which will then create a signed transaction which their Cardware device will split up into QR codes. The web wallet then scans these QR codes, decodes them and broadcasts the transaction.

---

# Documentation

## Initialization

### Code

```javascript
import Wallet from 'cardware-btc'; 
```

---

## New Wallet

This function initializes a wallet object in your web wallet. The xpub and the fingerprint are both received from the Cardware device after successfully pairing the web wallet and Cardware device. The pairing process involves scanning the **pair** QR codes from the Cardware device, extracting the xpub and fingerprint, then using them in creating the wallet object.

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| xpub | string | The xpub of the the hardware wallet. | ```"vpub5ZNhc5KKM6hACK6QDuo6UG1749XUeXf9Gbu8rcZQnNDeMJwUPrwzEVKsF7X7EzZe5yqwymfMA1tGJ9qAmjdmGHSkRW7SruCEDz9mgEkwWvN"``` |
| esplora_url | string | The address of the esplora you are using. | ```"https://blockstream.info/api"``` |
| fingerprint | string | The fingerprint used for identifying the correct xpub. | ```"fa436c5b"``` |
| network | string | The network you are using (mainnet or testnet). | ```"mainnet"``` |

### Code

```javascript
var wallet = await new Wallet(xpub, esplora_url, fingerprint, network);
```

### Output

No outputs.

---

## Sync

This function syncs your web wallet to make sure it has all the correct information to be able to get balances, construct unsigned transactions and broadcast transactions.

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

This function syncs your web wallet with max depth as a parameter to make sure it has all the correct information to be able to get balances, construct unsigned transactions and broadcast transactions.

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| max_depth | string | The max depth of the addresses you need to sync. | ```"m/0/0"``` |

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

This function estimates fees for a send transaction which takes a variable called **number of blocks** where the lower the number of blocks, the higher the estimated fee. Users can batch send transactions by populating multiple addresses and multiple amounts however the user must make sure both arrays have the same length.

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| addresses | array[string] | The addresses to send to. | ```["bc1q82d5gkw4xa0dgs55khfx0y4s7ntjwtfxu4h4fg"]``` |
| amounts | array[int64] | The send amounts in satoshis. | ```[1480]``` |
| number_of_blocks | int32 | The number of blocks for fee estimation. The lower the number, the higher the fee. | ```3``` |


### Code

```javascript
let result = wallet.estimate_fee(addresses, amounts, number_of_blocks);
```

### Output

The output is a uint64.

| Result | Description | Output |
|---|---|---|
| success | The fee estimation for a transaction (in satoshis). | ```1480``` |
| error | The addresses array and the amounts array are not the same length. | ```0``` |
| error | There is an issue parsing the network. | ```1``` |
| error | There is an invalid recipient address. | ```2``` |
| error | There is an issue fetching the fee estimates.. | ```3``` |
| error | There is insufficient BTC to make this transaction. | ```4``` |
| error | There are no UTXOs to spend. | ```5``` |

---

## Send

This function creates an unsigned transaction which it converts into a base64 string which it then splits up into chunks to be put into multiple QR codes. At the beginning of each chunk extra information is added. The extra information has the format of *(* + *index of QR code* + */* + *total QR codes* + *)* + *part of the unsigned transaction as a base64 string*.

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| addresses | array[string] | The addresses to send to. | ```["bc1q82d5gkw4xa0dgs55khfx0y4s7ntjwtfxu4h4fg"]``` |
| amounts | array[int64] | The send amounts (in satoshis). | ```[1000]``` |
| fee | int64 | The transaction fee worked out in estimate_fee (in satoshis). | ```1480``` |


### Code

```javascript
var qrcode_chunks = wallet.send(addresses, amounts, fee);
```

### Output

The output is an array of strings.

| Result | Description | Output |
|---|---|---|
| success | An array of base64 strings which can be shown as QR codes. | ```["(0/6)AgAAAAJCfJSUSIPEKOeG56APmMCEP6zPRPCz1/zyBsnFR5", "(1/6)gNNAAAAAAA/////4j8W+GTLq29Of7VdtqzMmkGpLJocgYd", "(2/6)1l/n4Crlse8vAAAAAAD/////AtAHAAAAAAAAFgAU3HfuEr", "(3/6)x48JEWN7r+DtOnmtCUXWBCWgAAAAAAABYAFDqbRFnVN17U", "(4/6)QpS10meSsPTXJy0mAAAAAA==:AAAAAOgDAAAAAAAAAAAAA", "(5/6)KhhAAAAAAAA"]``` |
| error | The addresses array and the amounts array are not the same length. | ```["Error: Recipients and amounts arrays must be the same length."]```
| error | The address is not associated with the BTC network. | ```["Error: Failed to parse network."]```
| error | There is insufficient BTC to make this transaction. | ```["Error: Insufficient funds."]```
| error | There are no UTXOs to spend. | ```["Error: No UTXOs to spend."]```
| error | There is an issue with the derivation path. | ```["Error: Derivation path error."]```
| error | There is an invalid recipient address. | ```["Error: Invalid recipient address."]```

---

## Broadcast

This function needs a signed transaction as a base64 string. It gets this by scanning the Cardware device. When scanning the QR codes of the signed transaction from the Cardware device it follows the format of *(* + *index of QR code* + */* + *total QR codes* + *)* + *part of the signed transaction as a base64 string*.

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| signed_transaction | string | The signed transaction in base64 that needs to be broadcasted. | ```"AgAAAAABAUJ8lJRIg8Qo54bnoA+YwIQ/rM9E8LPX/PIGycVHmA00AQAAAAD/////AugDAAAAAAAAFgAUOptEWdU3XtRClLXSZ5Kw9NcnLSabIwAAAAAAABYAFDWIiG3dUA5i2rzZg529bq4aQWPFAkgwRQIhAPk1OEfut27Z/YZsu5Xeik10inhcYYfXDXFRkOnqz/Y8AiBvx0Uqw3q3+LV8MA/cJZKComCL/2r/zXIx9cay94JXIwEhA4HigQXlfm+OMUk3YXW5cGxWOYZqfPzF0dMNLyWSHw+FAAAAAA=="``` |

### Code

```javascript
await wallet.broadcast(signed_transaction);
```

### Output

The output is a strings.

| Result | Description | Output |
|---|---|---|
| success | The transaction ID of the broadcasted transaction. | ```"340d9847c5c906f2fcd7b3f044cfac3f84c0980fa0e786e728c4834894947c42"``` |
| error | There is an issue parsing the base64 transaction string. | ```"Error: Failed to parse base64 transaction."```
| error | There is an issue decoding the hex transaction.| ```"Error: Decoding failed."```
| error | The signed transaction is invalid. | ```"Error: Invalid transaction."```
| error | There is an issue broadcasting the transaction. | ```"Error: Failed to broadcast transaction."```

---

## Address

This function returns the address of your Cardware device.

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
| success | The address of the wallet. | ```"bc1qxkygsmwa2q8x9k4umxpem0tw4cdyzc79kn5r5p"``` |
| error | There is an invalid extended public key. | ```"Error: Invalid extended public key."``` |
| error | There is an issue deriving the xpub. | ```"Error: Xpub derivation error."``` |

---

## New Address

This function returns the address of your Cardware device at a certain depth.

### Parameters

| Parameter | Type | Description | Example |
|---|---|---|---|
| derivation_path | string | The derivation path the new address must be derived from. | ```"m/0/0"``` |

### Code

```javascript
const result = wallet.new_address(derivation_path);
```

### Output

| Result | Description | Output |
|---|---|---|
| success | The address of the wallet. | ```"bc1qxkygsmwa2q8x9k4umxpem0tw4cdyzc79kn5r5p"``` |
| error | There is an invalid extended public key. | ```"Error: Invalid extended public key."``` |
| error | There is an issue deriving the xpub. | ```"Error: Xpub derivation error."``` |

---

## Balance

This function returns confirmed balance of your Cardware device.

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
| success | The confirmed balance of your wallet (in satoshis).| ```"11225"``` |
---

## Unconfirmed Balance

This function returns unconfirmed balance of your Cardware device.

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
| success | The unconfirmed balance of your wallet (in satoshis).| ```"1000"``` |

---