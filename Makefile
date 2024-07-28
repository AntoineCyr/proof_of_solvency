test: ## run tests
	cd circuits && mocha


__Nova__:
## Compile the Circuits for Nova
compile: 
	make compile-inclusion && make compile-liabilities-changes-folding

compile-inclusion:
	cd circuits && circom inclusion.circom --r1cs --sym --c --prime vesta && circom inclusion.circom --r1cs --sym --wasm --prime vesta

compile-liabilities-changes-folding:
	cd circuits && circom liabilities_changes_folding.circom --r1cs --sym --c --prime vesta && circom liabilities_changes_folding.circom --r1cs --sym --wasm --prime vesta

##Test the integration of the compiled circuits
nova-test:
	make liabilities-test && make inclusion-test

liabilities-test:
	cd nova && cargo run liabilities

inclusion-test:
	cd nova && cargo run inclusion
