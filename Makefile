export FORCE_COLOR = true

test-quest-1-matrix-transposition:
	cd quest-1/matrix-transposition && cargo test --lib

test-quest-1-find-factorial:
	cd quest-1/find-factorial && cargo test --lib

test-quest-1-reverse-string:
	cd quest-1/reverse-string && cargo test --lib

test-quest-1-fibonacci:
	cd quest-1/fibonacci2 && cargo test --lib

test-quest-1-scalar:
	cd quest-1/scalar && cargo test --lib

test-quest-1-looping:
	cd quest-1/looping && cargo test --lib

test-quest-1-temperature-conv:
	cd quest-1/temperature-conv && cargo test --lib

test-quest-1-speed-transformation:
	cd quest-1/speed-transformation && cargo test --lib

test-quest-1:
	make test-quest-1-fibonacci
	make test-quest-1-scalar
	make test-quest-1-temperature-conv
	make test-quest-1-looping
	make test-quest-1-speed-transformation
	make test-quest-1-reverse-string
	make test-quest-1-find-factorial
	make test-quest-1-matrix-transposition

test:
	make test-quest-1
	