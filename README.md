# Integration of the Nova Folding Scheme with Proof of Solvency Circuits

This project demonstrates the integration of the Nova folding scheme with proof of solvency circuits to significantly reduce the computational workload required for verifying multiple Merkle sum trees and balance inclusions.

## Circuits Description

### Liabilities

The proof of liabilities operates on a list of balances and a list of email hashes as private inputs. The main purposes of the circuit are:

1. **Validation:** Ensure all values are non-negative and fall within a specified range to prevent overflow or underflow issues, given that the operations occur within a finite field.
2. **Merkle Tree Construction:** Construct a Merkle tree and output the total balance sum and the root hash of the Merkle tree.

### Liabilities Changes

In our modified circuit, we adjust the Merkle Tree inside the circuit. For every change, the corresponding Merkle Path is sent and verified by the circuit. The circuit then computes a new Root Hash for each change and outputs the final Merkle Hash.

### Nova Folding Scheme Circuits

#### Liabilities Changes Folding

To implement folding, we slightly adjust the way we build the changes circuit. Everything except the way we handle inputs and outputs stays the same. The private inputs vary for every instance, while the public inputs are carried over from round to round.

#### Inclusion

The proof of inclusion aims to prove that the balance of a user is included in the Merkle Tree created in the proof of liabilities. To prove that a balance is included, it is sufficient to show that you know the Merkle path of a user balance. Using the Nova folding scheme, we can prove the balance of a user is included at multiple points in time. For instance, we can have 365 steps, one for each day, to prove that the balance of the user was included every day in the last year.

## Run the Circuits Tests

```sh
make test
```

## Compile the Circuits

```sh
make compile
```

## How to Run the Integration of the Folding Circuit with Nova

```sh
make liabilities test
make inclusion test
```

### Additional Information

For a more in-depth understanding of the circuits, including performance and optimization analysis, please refer to my upcoming thesis.
