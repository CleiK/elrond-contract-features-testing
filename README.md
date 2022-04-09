# elrond-contract-features-testing

This repository contains contracts aiming to test and better understand elrond contracts features

## events-tester

Testing the event (log) system

- Emit a single event on an endpoint
- Emit two events on an endpoint
- Emit a single event in an async call callback
- Emit two events in an async call callback

This uses a delegation-mock you can find in mocks/delegation-mock to perform asynchronous call.
