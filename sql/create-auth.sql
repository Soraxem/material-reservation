-- define schema auth

CREATE SCHEMA auth AUTHORIZATION pg_database_owner;

-- auth.users definition

CREATE TABLE auth.users (
	pk uuid DEFAULT uuidv4() NOT NULL,
	"name" text NOT NULL,
	hash text NULL,
	CONSTRAINT users_pk PRIMARY KEY (pk)
);