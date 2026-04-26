-- inventory schema definition

CREATE SCHEMA inventory AUTHORIZATION pg_database_owner;

COMMENT ON SCHEMA inventory IS 'material amounts management';

-- inventory.articles definition

CREATE TABLE inventory.articles (
	pk uuid DEFAULT uuidv4() NOT NULL,
	"name" text NOT NULL,
	description text NULL,
	CONSTRAINT articles_pk PRIMARY KEY (pk)
);

-- inventory.positions definition

CREATE TABLE inventory.positions (
	pk uuid DEFAULT uuidv4() NOT NULL,
	fk_article uuid NOT NULL,
	is_consumable bool NOT NULL,
	is_unique bool NOT NULL,
	amount int4 NOT NULL, -- only 0 or 1 if position is unique, otherwise amount
	normal_amount int4 NULL, -- ignore when position is unique
	unique_name text NULL, -- ignore when position not unique
	CONSTRAINT position_registry_pk PRIMARY KEY (pk),
	CONSTRAINT position_registry_articles_fk FOREIGN KEY (fk_article) REFERENCES inventory.articles(pk)
);

-- Column comments

COMMENT ON COLUMN inventory.positions.amount IS 'only 0 or 1 if position is unique, otherwise amount';
COMMENT ON COLUMN inventory.positions.normal_amount IS 'ignore when position is unique';
COMMENT ON COLUMN inventory.positions.unique_name IS 'ignore when position not unique';

-- inventory.article_relations definition

CREATE TABLE inventory.article_relations (
	from_article_id uuid NOT NULL,
	to_article_id uuid NOT NULL,
	CONSTRAINT articele_relations_pk PRIMARY KEY (from_article_id, to_article_id),
	CONSTRAINT from_article_fk FOREIGN KEY (from_article_id) REFERENCES inventory.articles(pk) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT to_article_fk FOREIGN KEY (to_article_id) REFERENCES inventory.articles(pk) ON DELETE CASCADE ON UPDATE CASCADE
);
COMMENT ON TABLE inventory.article_relations IS 'Defines directional position relations, as recommendations for users.';


-- inventory.article_list view

CREATE OR REPLACE VIEW inventory.article_list
AS SELECT articles.name,
    positions.amount
   FROM inventory.positions
     JOIN inventory.articles ON positions.fk_article = articles.pk;

COMMENT ON VIEW inventory.article_list IS 'Display position amounts with their name';