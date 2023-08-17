-- Add up migration script here
CREATE TABLE contacts.categories (
    id integer NOT NULL,
    category character varying(100)
);

ALTER TABLE ONLY contacts.categories
    ADD CONSTRAINT groups_pkey PRIMARY KEY (id);
