### PPIMS in Rust

this is just a toy project to rebuild the PPIMS dummy server that I built in python for the DCMS project.
Using this I wanna learn handling http requests, basic serialization and deserialization, file handling, etc in rust.

#### Items

- [x] basic get & post server
- [x] define default endpoints
- [x] request json pretty printer
- [x] deserialize with custom field names
- [ ] error logs in server side console
- [x] store customer data in file
- [x] load data on startup
- [ ] have ppims headers
- [x] have mobile number for customer dedupe check
- [ ] have the KYC restrictions enabled
- [ ] no std logs or err logs, full async
- [x] optional json params
- [X] document the request response messages
- [X] complete all request response messages
- [X] pretty print json request - response
- [ ] frontend in react for PPIMS
- [X] async APIs for the react UI - sign-in with hardcoded credentials
- [ ] UI Home page - Top customers, all Customers
- [ ] UI transactions - sort from last received to backwards
- [ ] UI transactions - list by transaction type(all, create customer, update limit, etc)
- [ ] UI transactions - filter transactions by time frame
- [ ] UI search by customer name, mobile number
- [ ] UI list transaction at customer section
- [ ] embedded SQLite database
- [ ] use async querying?
- [ ] multiple channel support for customer
- [ ] fill other data in the customer registration status response
- [ ] add check limit request response
- [ ] add update limit request response

#### Things to learn in the process

- Building a basic http server
- have endpoints to respond in json
- file handling
- async in rust
- error handling
- code separation

#### Other items

- static code in rust
- mod inside a .rs file