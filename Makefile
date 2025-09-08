test: ## run tests
	cd circuits/test && ../../node_modules/.bin/mocha *.js

clean: ## clean compiled circuits
	rm -rf circuits/compile/

help: ## display this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'


__Nova__:
## Compile the Circuits for Nova
compile: ## compile all circuits to circuits/compile/
	make compile-inclusion && make compile-liabilities-changes-folding && make compile-liabilities-changes && make compile-liabilities

compile-inclusion:
	cd circuits && mkdir -p compile && circom inclusion.circom --r1cs --sym --c --prime vesta -l ../node_modules -o compile && circom inclusion.circom --r1cs --sym --wasm --prime vesta -l ../node_modules -o compile

compile-liabilities-changes-folding:
	cd circuits && mkdir -p compile && circom liabilities_changes_folding.circom --r1cs --sym --c --prime vesta -l ../node_modules -o compile && circom liabilities_changes_folding.circom --r1cs --sym --wasm --prime vesta -l ../node_modules -o compile

compile-liabilities-changes:
	cd circuits && mkdir -p compile && circom liabilities_changes.circom --r1cs --sym --c --prime bn128 -l ../node_modules -o compile && circom liabilities_changes.circom --r1cs --sym --wasm --prime bn128 -l ../node_modules -o compile

compile-liabilities:
	cd circuits && mkdir -p compile && circom liabilities.circom --r1cs --sym --c --prime bn128 -l ../node_modules -o compile && circom liabilities.circom --r1cs --sym --wasm --prime bn128 -l ../node_modules -o compile

##Test the integration of the compiled circuits
nova-test:
	make liabilities-test && make inclusion-test

liabilities-test:
	cd nova && cargo run liabilities

inclusion-test:
	cd nova && cargo run inclusion
