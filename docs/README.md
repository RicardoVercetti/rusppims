## Rusppims

### About
The PPI dubbed to prepaid payment instrument is the RBI system to regulate limits in spendings on Indian government regulated financial processors. PPI has hold of customer information, account information, spending, and the maximum amount the customer can possibly spend on one month.

### Overview

The rust implementation of something called a prepaid system(PPIMS)
Over 6 APIs are hosted for the purpose of maintaining the transaction guide line limits of RBI.

### Usage

The Customer have to be on-boarded and then transactions can be made for the customer for a specific KYC.
There are 3 different KYC options available - Min KYC, New Min KYC & full KYC.
Full KYC has no limits while the min KYC has max spend of 10_000 per month.

### Available APIs

1. Add customer
2. Update customer
3. Customer registration status
4. Check customer KYC
5. Check customer limit
6. Update customer limit
