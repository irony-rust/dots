# Additional Services Generic Types

Generic types for Events Data.

#### Status Type

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| status* | string [10] | success \| failed |
| error_message | string [1000] | If error message with full error |
| error_type | string [50] | Description of type error |
 
#### Token Info Type

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| token | string [32] | AES256 token for authentication |
| expired | string [50] | Expiration token time |
| carrier_id | string [50] | Carrier ID |
| device_id | string [50] | Carrier Device ID |