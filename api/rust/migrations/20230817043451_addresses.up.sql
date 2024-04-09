-- Add up migration script here
CREATE TABLE contacts.addresses (
    id_user integer NOT NULL,
    address character varying(250) NOT NULL
);

ALTER TABLE ONLY contacts.addresses
    ADD CONSTRAINT fk_id_customer FOREIGN KEY (id_user) REFERENCES contacts.users(id);

