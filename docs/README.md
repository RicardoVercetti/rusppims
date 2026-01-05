## Rusppims

### About
The PPI dubbed prepaid payment instrument is the RBI system to regulate limits in spendings on Indian government regulated financial processors. PPI has hold of customer information, account information, spending, and the maximum amount the customer can possibly spend on a month based on the KYC of the customer.

### Overview

The rust implementation of something called a prepaid system(PPIMS)
Over 6 APIs are hosted for the purpose of maintaining the transaction guide line limits of RBI.

### Usage

- The Customer have to be on-boarded using the [Add customer](./APIs/add_customer.md) API and then transactions can be made for the customer for a specific KYC.
- Here are 3 different KYC options available - Min KYC, New Min KYC & full KYC.
Full KYC has no limits while the min KYC has max spend of 10_000 per month.
- When any payment processors(CMS systems, core banking systems, etc.,) allow any type of payments, they have to call the [Check customer limit](./APIs/check_customer_limit.md) PPIMS API, and after processing the transaction(i.e. post Credit or Debit), they have to call [Update customer limit](./APIs/update_customer_limit.md) API to let the PPIMS know that a certain transaction is done against a customer.
- This is useful for tracking spendings when the customer creates accounts and maintains balance in multiple Financial Institutions. 

### Available APIs

1. [Add customer](./APIs/add_customer.md)
2. [Update customer](./APIs/update_customer.md)
3. [Customer registration status](./APIs/customer_registration_status.md)
4. [Check customer KYC](./APIs/check_customer_kyc.md)
5. [Check customer limit](./APIs/check_customer_limit.md)
6. [Update customer limit](./APIs/update_customer_limit.md)

### The Goal

- [ ] Have the basic PPIMS functionality to server as a dummy for DCMS(the project I'm working on professionally)
- [ ] Test the smoothness of applications built in rust.
- [ ] Tryout simple and minimal UI building
- [ ] Try huge amount(in lakhs) of data using SQLite
- [ ] Load test rust's Tokyo runtime(perhaps compare the resource usage of this Vs same application built in JS runtime) under heavy stress on low end devices