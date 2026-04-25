-- define schema transactions

CREATE SCHEMA transactions AUTHORIZATION pg_database_owner;

COMMENT ON SCHEMA transactions IS 'borrowing and usage of material by users';

-- transactions.reservations definition

CREATE TABLE transactions.reservations (
	pk uuid DEFAULT uuidv7() NOT NULL,
	fk_users uuid NOT NULL,
	"from" timestamp NOT NULL,
	"until" timestamp NULL, -- Only needed, if non consumable items are reserved.
	description text NULL,
	CONSTRAINT reservations_pk PRIMARY KEY (pk)
);

ALTER TABLE transactions.reservations ADD CONSTRAINT reservations_users_fk FOREIGN KEY (fk_users) REFERENCES auth.users(pk);

COMMENT ON COLUMN transactions.reservations."until" IS 'Only needed, if non consumable items are reserved.';

-- transactions.reservation_items definition

CREATE TABLE transactions.reservation_items (
	item_id uuid NOT NULL,
	reservation_id uuid NOT NULL,
	amount int4 NULL,
	CONSTRAINT reservation_items_pk PRIMARY KEY (item_id, reservation_id)
);
COMMENT ON TABLE transactions.reservation_items IS 'Join items to reservations';

-- transactions.reservation_items foreign keys

ALTER TABLE transactions.reservation_items ADD CONSTRAINT items_fk FOREIGN KEY (item_id) REFERENCES inventory.items(pk);
ALTER TABLE transactions.reservation_items ADD CONSTRAINT reservations_fk FOREIGN KEY (reservation_id) REFERENCES transactions.reservations(pk);
