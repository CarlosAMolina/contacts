-- Add up migration script here
CREATE TABLE contacts.nicknames (
    id_user integer NOT NULL,
    nickname character varying(50) NOT NULL
);

ALTER TABLE ONLY contacts.nicknames
    ADD CONSTRAINT fk_id_customer FOREIGN KEY (id_user) REFERENCES contacts.users(id);

