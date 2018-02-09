# Location Service

Location service is responsible for CRUD (create, read, update, delete) opperations with carriers GEO data.

## Events for Receive
* [Location](#location-event)
* [Get Latest](#get-latest-event)


### Location Event

#### Request Fields:

| Field | Type [min/length/max] | Description | 
| --- | --- | --- |
| location* | [location_type](AdditionaServicesGenericTypes.md#location-type) | Location Data from carrier device |
| carrier_id* | string [100] | Carrier ID |

\* required

####  Response Fields:

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| status* | [status_type](AdditionaServicesGenericTypes.md#status-type) | Response status |

\* required 


### Get Latest Event

#### Request Fields:

| Field | Type [min/length/max] | Description | 
| --- | --- | --- |
| carrier_ids* | []string [50] | Array of carrier IDs |
| newer_then | int [64] | Number of minutes that will be taken as initial value for time boundary |

\* required

####  Response Fields:

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| locations | [string] [location_type](AdditionaServicesGenericTypes.md#location-type) | hash table with carrier ID as key and Location as value |

\* required 
