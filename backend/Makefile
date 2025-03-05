create-mig:
	diesel migration generate --diff-schema=src/tables.rs $(name)

run-mig:
	diesel migration run

run:
	cargo run -- --config-path="./configs/dev.toml" --secret-path="./secrets"

watch:
	cargo watch -x 'run -- --config-path="configs/dev.toml" --secret-path="secrets"'