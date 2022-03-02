export FORCE_COLOR = true


test-quest-1-fibonacci:
	cd quest-1/fibonacci2 && cargo test --lib

test-quest-1-scalar:
	cd quest-1/scalar && cargo test --lib

test-quest-1:
	make test-quest-1-fibonacci
	make test-quest-1-scalar

test:
	make test-quest-1
	