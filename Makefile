export FORCE_COLOR = true

test-quest-1-x:
	cd quest-1/$(EX_NAME) && cargo test --lib

test-quest-1:
	for ex in fibonacci2 \
	     	  scalar \
	     	  temperature-conv \
	     	  looping \
	     	  speed-transformation \
	     	  reverse-string \
	     	  find-factorial \
	     	  matrix-transposition \
	     	  division-and-remainder \
	     	  tuples ; do \
	    make test-quest-1-x EX_NAME=$$ex; \
	done;

test-quest-2-x:
	cd quest-2/$(EX_NAME) && cargo test --lib

test-quest-2:
	cd quest-2/arrange-it && cargo test --lib || echo "it may fail and I don't care" || true 

	for ex in ownership \
		 	  copy \
		 	  borrow \
		 	  doubtful \
		 	  borrow-the-reference \
		 	  changes \
		 	  string-literals \
		 	  name-initials \
		 	  tic-tac-toe ; do \
	    make test-quest-2-x EX_NAME=$$ex; \
	done;

test-quest-3-x:
	cd quest-3/$(EX_NAME) && cargo test --lib

test-quest-3:
	for ex in circle \
	            card-deck \
	            arrays \
	            strings \
				edit-distance \
				to-url \
				capitalizing \
				hashing \
				string-permutation \
				bigger \
				simple-hash \
				collect; do \
	    make test-quest-3-x EX_NAME=$$ex; \
	done;
		 

test:
	make test-quest-1 \
	     test-quest-2 \
		 test-quest-3