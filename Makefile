RUST_LOG=debug

debug:
	RUST_LOG=$(RUST_LOG) cargo run -r

ENVIRONMENT=test

test:
	ENVIRONMENT=$(ENVIRONMENT) cargo test -- --nocapture

migrate-up:
	cd migration && cargo run up -u ${DATABASE_URL}

migrate-fresh:
	cd migration && cargo run fresh -u ${DATABASE_URL}
