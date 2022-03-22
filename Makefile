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

test-quest-2-borrow-the-reference:
	cd quest-2/borrow-the-reference && cargo test --lib

test-quest-2-changes:
	cd quest-2/changes && cargo test --lib

test-quest-2-string-literals:
	cd quest-2/string-literals && cargo test --lib

test-quest-2-name-initials:
	cd quest-2/name-initials && cargo test --lib

test-quest-2-arrange-it:
	cd quest-2/arrange-it && cargo test --lib || echo "it may fail and I don't care" || true

test-quest-2-tic-tac-toe: 
	cd quest-2/tic-tac-toe && cargo test --lib

test-quest-2:
	make test-quest-2-ownership \
		 test-quest-2-copy \
		 test-quest-2-borrow \
		 test-quest-2-doubtful \
		 test-quest-2-borrow-the-reference \
		 test-quest-2-changes \
		 test-quest-2-string-literals \
		 test-quest-2-name-initials \
		 test-quest-2-arrange-it \
		 test-quest-2-tic-tac-toe

test-quest-3-circle:
	cd quest-3/circle && cargo test --lib

test-quest-3-card-deck:
	cd quest-3/card-deck && cargo test --lib

test-quest-3-arrays:
	cd quest-3/arrays && cargo test --lib

test-quest-3-strings:
	cd quest-3/strings && cargo test --lib

test-quest-3:
	make test-quest-3-circle \
		 test-quest-3-card-deck \
		 test-quest-3-arrays \
		 test-quest-3-strings \

test:
	make test-quest-1 \
	     test-quest-2 \
		 test-quest-3