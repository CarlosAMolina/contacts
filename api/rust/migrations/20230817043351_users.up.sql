-- Add up migration script here
CREATE TABLE contacts.users (
    id serial NOT NULL,
    name character varying(50) NOT NULL,
    surname character varying(50)
);

ALTER TABLE ONLY contacts.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);

