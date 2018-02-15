# Auth Service

Auth Service reponse for login, registration, 
and Carrier identification.

## Events for Receive
* [Login](#login-event)
* [Registration](#registration-event)
* [GetToken](#gettoken-event)
* [Logout](#logout-event)
* [ForgotPassword](forgotpassword-event)

### Login Event

#### Request Fields:

| Field | Type [min/length/max] | Description | 
| --- | --- | --- |
| usernamee* | string [255] | Carrier user name |
| password* | string [255] | Carrier password |
| device_id* | string [100] | Carrier Device ID | 
\* reqiered

####  Response Fields:

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| status* | [status_type](AdditionaServicesGenericTypes.md#status-type) | Response status |
| token_info | [token_info_type](AdditionaServicesGenericTypes.md#token-info-type) | Token information for User |
\* reqiered 

### Registration Event

#### Request Fields:

| Field | Type [min/length/max] | Description | 
| --- | --- | --- |
| usernamee* | string [255] | Carrier user name |
| password* | string [255] | Carrier password |
| device_id* | string [100] | Carrier Device ID | 
\* reqiered

####  Response Fields:

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| status* | [status_type](AdditionaServicesGenericTypes.md#status-type) | Response status |
| token_info | [token_info_type](AdditionaServicesGenericTypes.md#token-info-type) | Token information for User |
\* reqiered 

### GetToken Event

#### Request Fields:

| Field | Type [min/length/max] | Description | 
| --- | --- | --- |
| device_id* | string [100] | Carrier Device ID | 
\* reqiered

####  Response Fields:

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| status* | [status_type](AdditionaServicesGenericTypes.md#status-type) | Response status |
| token_info | [token_info_type](AdditionaServicesGenericTypes.md#token-info-type) | Token information for User |
\* reqiered 

### Logout

#### Request Fields:

| Field | Type [min/length/max] | Description | 
| --- | --- | --- |
| device_id* | string [100] | Carrier Device ID | 
\* reqiered

####  Response Fields:

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| status* | [status_type](AdditionaServicesGenericTypes.md#status-type) | Response status |
\* reqiered 


### ForgotPassword

#### Request Fields:

| Field | Type [min/length/max] | Description | 
| --- | --- | --- |
| usernamee* | string [255] | Carrier user name |
| device_id* | string [100] | Carrier Device ID | 
\* reqiered

####  Response Fields:

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| status* | [status_type](AdditionaServicesGenericTypes.md#status-type) | Response status |
\* reqiered 
