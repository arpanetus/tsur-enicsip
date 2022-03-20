export FORCE_COLOR = true

test-quest-1-tuples:
	cd quest-1/tuples && cargo test --lib

test-quest-1-division-and-remainder:
	cd quest-1/division-and-remainder && cargo test --lib

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
	make test-quest-1-fibonacci \
	     test-quest-1-scalar \
	     test-quest-1-temperature-conv \
	     test-quest-1-looping \
	     test-quest-1-speed-transformation \
	     test-quest-1-reverse-string \
	     test-quest-1-find-factorial \
	     test-quest-1-matrix-transposition \
	     test-quest-1-division-and-remainder \
	     test-quest-1-tuples

test-quest-2-ownership:
	cd quest-2/ownership && cargo test --lib

test-quest-2-copy:
	cd quest-2/copy && cargo test --lib

test-quest-2-borrow:
	cd quest-2/borrow && cargo test --lib

test-quest-2-doubtful:
	cd quest-2/doubtful && cargo test --lib

test-quest-2:
	make test-quest-2-ownership \
		 test-quest-2-copy \
		 test-quest-2-borrow \
		 test-quest-2-doubtful

test:
	make test-quest-1 \
	     test-quest-2
	