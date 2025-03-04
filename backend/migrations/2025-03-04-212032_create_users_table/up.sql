-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"auth_provider_id" VARCHAR NOT NULL UNIQUE,
	"email" VARCHAR NOT NULL UNIQUE,
	"first_name" VARCHAR,
	"last_name" VARCHAR,
	"is_active" BOOL NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL
);

CREATE INDEX "users_email_index" ON "users"("email");
CREATE INDEX "users_auth_provider_id_index" ON "users"("auth_provider_id");

