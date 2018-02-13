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

#### Profile Type

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| first_name* | string [50] | First Name |
| last_name* | string [50] | Last Name |
| license_num | string [100] | Number of drivers license |
| license_exp | string [20] | Date when DL expires |
| birth_date | string [20] | Drivers birth date |
| address | string [50] | Drivers address |
| post_index | string [10] | Drivers Post Index |
| license_url | string [100] | Url of drivers license scan/photo |
| vehicle_url | string [100] | Url of drivers vehicle license scan/photo |
| phone* | string [20] | Drivers contact phone |
| approved | bool | Whether or not drivers info approved by managers |

#### Location type

| Field | Type [min/length/max] | Description | 
| --- | --- | --- |
| lat* | float [64] | Latitude |
| lon* | float [64] | Longitude |
| accuracy* | float [64] | Accuracy of the location data (lower is better) |
