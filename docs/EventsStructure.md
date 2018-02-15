# General API documentation

## Events Structure

### Event Request

All Events should have basic structure described below.
Event message should represent JSON object with valida JSON data.

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| event_name* | string [100] | Unique event name for logic segment |
| event_created_at* | string [50] | Time of event creation UTC |
| chain_from | []string [100] | Сhain of service names through which the event has already passed. Service that send pass event to another service should add name to right of secuence. Service that send pass event to other service should add name to right of sequence. |
| chain_to | []string [100] | Сhain of service names for discovering destination service. If event pass through the service, service name should be deleted from sequence. Destination is form left to right. Right name is end destination service. |
| data* | []object | Main event data |
\* required

#### Example

```json
{
	"event_name": "Login",
	"event_created_at": "2018-02-07 20:49:38.78637637 +0000 UTC",
	"chain_from": ["aws-1401.broker", "aws-33011.carrier"],
	"chain_to": ["aws-20011.login"],
	"data": [{
		"username": "test",
		"password": "test"
	}]
}
```

### Event Response

All events should have response. Basic structure described below.
Event message should represent JSON object with valida JSON data.

| Field | Type [min/length/max] | Description |
| --- | --- | --- |
| event_name* | string [100] | Same as for Event Request |
| "event_response_at"* | string [50] | Time of event response UTC |
| chain_from | []string [100] | Same as for Event Request |
| chain_to | []string [100] | Same as for Event Request |
| status* | string [20] | Event status: success \| failed \| other special status specified by event |
| errors | []string [1000] | If some errors appeared it should contain map of error. Is should be logical related with status field |
| data | []object | Main event data response |

\* required

#### Example

```json
{
	"event_name": "Login",
	"event_response_at": "2018-02-07 20:49:38.78637637 +0000 UTC",
	"chain_from": ["aws-1401.broker", "aws-33011.carrier"],
	"chain_to": ["aws-20011.login"],
	"status": "success",
	"data": [{
		"token": "gdee-1234-assqwwe-16388-qwerrwe-1247"
	}]
}
```
