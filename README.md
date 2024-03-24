# Diploma Verification using Zero-Knowledge Proofs
This Rust code demonstrates a simple example of using zero-knowledge proofs (ZKPs) to verify a diploma. In this scenario, we have a diploma circuit that checks if a person has a diploma from a particular university with a grade above a certain threshold. We utilize the bellman and pairing crates for ZKP functionalities.

### Usage
To run the code:

Ensure you have Rust installed on your system.
Clone this repository.
Navigate to the project directory.
Run cargo build to compile the code.
Execute the binary produced in the target/debug directory.
### Components
DiplomaCircuit
DiplomaCircuit is a struct representing the circuit for diploma verification. It takes generic field F where Field is a trait representing finite fields.
The circuit checks if the university is valid (encoded as A, B, or C) and if the grade is above a specified threshold.
Implements the Circuit trait, providing a method synthesize for circuit synthesis.
### Main Function
In the main function, parameters for the ZKP system are generated using generate_random_parameters.
A proof is then created for a given set of inputs (university and grade) using create_random_proof.
The proof is verified using verify_proof with prepared verifying key and public inputs.
### Dependencies
bellman: A crate providing functionality for building zk-SNARK circuits.
pairing: Part of the bellman ecosystem, providing pairing-based cryptographic operations.
rand: A random number generation library.
Example
This code example demonstrates how to utilize ZKPs to verify diploma credentials. In the provided scenario, the proof is generated for a diploma from University A with a passing grade. However, this can be extended to various diploma verification scenarios by modifying the input parameters and constraints in the DiplomaCircuit.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
This code is a simple demonstration inspired by real-world applications of zero-knowledge proofs.
Special thanks to the contributors of the bellman and pairing crates for providing powerful tools for implementing ZKPs in Rust.
Feel free to contribute, report issues, or suggest improvements!
