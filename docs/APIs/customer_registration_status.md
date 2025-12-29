# Check Customer request

### Usage

- This tells if a customer is already created. If it is already present, create customer will always fail
- If a `500` is returned, then the customer is not found(when searched using mobile number), therefore it can be created with a AddCustomer request
- If a `000` is returned, it means the customer already exists in PPIMS and cannot be created(i.e. the AddCustomer will always fail)

### sample request

```json
{
	"Data": {
		"Check_Reg_Status": {
			"Ref_Id": "123456789871234",
			"Mobile_Number": "919952324523",
			"Name": "Chandrasekar C",
			"Date_Of_Birth": "08-02-1994",
			"Ovid_Type": "ADR",
			"Ovid_Value": "123456789012",
			"System_Id": "ENT"
		},
		"Username": "PPIM_ENT",
		"Password": "ENT123"
	},
	"Risk": {}
}
```

### sample response

```json
 {
    "Data": {
        "Resp_Code": "500",
        "Unique_Id": "NA",
        "KYC_Flag": "NA",
        "KYC_Updated_Channel": "NA",
        "KYC_Updated_On": "NA",
        "Cif_Id": "NA",
        "Remaining_Avail_Limit": "10000.00",
        "Utilized_Bal": "0.00"
    },
    "Risk": {},
    "Links": {},
    "Meta": {}
}
```