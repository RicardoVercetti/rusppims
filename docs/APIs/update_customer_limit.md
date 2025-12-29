### Usage
- This request is sent after the transaction is processed(amount is credited/debited)

### Sample Request
```json
{
	"Data": {
		"Update_Limit": {
			"Ref_Id": "123456789871234",
			"Unique_Id": "515132527961234",
			"Amount": "2000.00",
			"Tran_Status": "S",
			"Tran_Type": "D",
			"Avail_Bal": "4000.00",
			"Tran_Id": "123456789012",
			"System_Id": "ENT",
			"Enquiry_Ref_Id": "123456789871233"
		},
		"Username": "PPIM_ENT",
		"Password": "ENT123"
	},
	"Risk": {}
}
```

### Sample Response
```json
{
	"Data": {
		"Resp_Code": "000",
		"Cumulative_Bal": "1000.00",
		"Avail_Limit": "2000.00"
	},
	"Risk": {},
	"Links": {},
	"Meta": {}
}
```