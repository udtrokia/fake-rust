### Crypto

#### 1. generate_keypair
##### Interface Function
> generate random keypair use OsRng.

##### Request Parameters
|parameter|required|type|instructions|
|:-----|:-----|:-----|-----|

##### Return Field
|return field|field type|
|:-----|:-----|
|keypair_bytes|[u8, 64]|
> [0..32] is secret_key, [32..64] is public key

#### 2. generate_publickey_from_secretkey
##### Interface Function
> generate public key by input secret key.

##### Request Parameters
|parameter|required|type|instructions|
|:-----|:-----|:-----|-----|
|secret_key_bytes |true| [u8;32]|secret key|

##### Return Field
|return field|field type|
|:-----|:-----|
|public_key_bytes |[u8, 32]|

#### 3. signature
##### Interface Function
> signaure a message by secret key.

##### Request Parameters
|parameter|required|type|instructions|
|:-----|:-----|:-----|-----|
|message |true| &str |message need sign|
|secret_key_bytes |true|[u8;32]|secret key|

##### Return Field
|return field|field type|
|:-----|:-----|
|signaure_bytes |[u8, 64]|

#### 4. verify
##### Interface Function
> verify signature use public key.

##### Request Parameters
|parameter|required|type|instructions|
|:-----|:-----|:-----|-----|
|message |true| &str |message need sign|
|signature_bytes |true|[u8;64]|signature|
|public_key_bytes |true|[u8;32]|public key|

##### Return Field
|return field|field type|
|:-----|:-----|
|is_verify | bool |


