create-mig:
	diesel migration generate --diff-schema=src/tables.rs $(name)

run-mig:
	diesel migration run