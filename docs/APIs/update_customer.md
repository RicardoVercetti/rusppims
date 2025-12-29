### Usage
- Used for updating the KYC or other customer data like Aadhar, mobile number, etc.,

### Sample Request
```json
{
    "Data": {
        "Update_Customer": {
            "Ref_Id": "123456789871234",
            "Unique_Id": "515132527961234",
            "Mobile_Number": "NA",
            "Customer_Name": "NA",
            "Date_Of_Birth": "NA",
            "Email_Id": "NA",
            "Account_Number": "568471265745",
            "Account_Status": "A",
            "Card_Number": "NA",
            "Card_Status": "NA",
            "KYC_Flag": "NA",
            "KYC_Updated_Channel": "NA",
            "KYC_Updated_On": "NA",
            "Maiden_Name": "NA",
            "Ovid_Type": "NA",
            "Ovid_Value": "NA",
            "System_Id": "ENT",
            "Cif_Id": "NA"
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
        "Unique_Id": "515427983121236",
        "Old_Unique_Id": "NA"
    },
    "Risk": {},
    "Links": {},
    "Meta": {}
}
```