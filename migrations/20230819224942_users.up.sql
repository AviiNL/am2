-- Add up migration script here

CREATE TABLE "users" (
	"id"	UUID NOT NULL,
	"name"	TEXT NOT NULL,
	"email"	TEXT NOT NULL UNIQUE,
	"verified"	BOOL NOT NULL DEFAULT false,
	"password"	TEXT NOT NULL,
	"roles"	TEXT NOT NULL DEFAULT '',
	"created_at"	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"updated_at"	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY("id")
);

CREATE INDEX users_email_idx ON users (email);

CREATE TABLE "user_tokens" (
    "token"	        UUID NOT NULL,
    "user_id"	    UUID NOT NULL,
    "ip"            TEXT NOT NULL,
	"created_at"	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"last_used"	    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY("token"),
    FOREIGN KEY("user_id") REFERENCES "users"("id")
);