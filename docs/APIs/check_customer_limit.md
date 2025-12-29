### Usage
- sent before sending the update customer limit.
- if the `Resp_Code` is `000` and `Allow_Customer` is `T`, its okay to do the debit/credit for the customer.
- If `T` is the response, a successive [update_customer_limit](update_customer_limit.md) will be sent.

### Sample Request
```json
{
	"Data": {
		"Check_Limit": {
			"Ref_Id": "123456789871234",
			"Unique_Id": "515132527961234",
			"Account_Number": "123456789012",
			"Card_Number": "NA",
			"Amount": "2000.00",
			"Tran_Type": "D",
			"Avail_Bal": "2000.00",
			"System_Id": "ENT"
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
		"Unique_Id": "515132527961234",
		"Allow_Customer": "T",
		"Cumulative_Bal": "6000.00",
		"Avail_Amount_Limit": "4000.00",
		"Cif_Id": "NA",
		"Old_Unique_Id": "NA"
	},
	"Risk": {},
	"Links": {},
	"Meta": {}
}
```