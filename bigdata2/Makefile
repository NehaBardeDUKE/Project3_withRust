format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

release:
	cargo lambda build --release --x86_64

deploy:
	cargo lambda deploy

invoke:
	cargo lambda invoke --remote \
  		--data-ascii '{"stmt": "Hey how r u"}' \
  		--output-format json \
  		bg_with_trigger
			
all: format lint test run