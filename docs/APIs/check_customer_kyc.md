### Usage
- can be used to to get the KYC status of customer(not really used much though)

### Sample request
```json
{
	"Data": {
		"Check_KYC": {
			"Ref_Id": "123456789871234",
			"Unique_Id": "515132527961234",
			"System_Id": "ENT"
		},
		"Username": "PPIM_ENT",
		"Password": "ENT123"
	},
    "Risk": {}
}
```

### Sample response
```json
{
	"Data": {
		"Resp_Code": "000",
		"KYC_Flag": "P",
		"KYC_Updated_Channel": "PGO",
		"KYC_Updated_On": "01-01-2018 11:00:00",
		"Cif_Id": "NA",
		"Unique_Id": "115427983121234",
		"Old_Unique_Id": "NA"
	},
	"Risk": {},
	"Links": {},
	"Meta": {}
}
```