-- Add up migration script here
CREATE TABLE contacts.addresses (
    id_user integer NOT NULL,
    address character varying(250) NOT NULL
);

