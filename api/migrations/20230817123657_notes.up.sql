-- Add up migration script here
CREATE TABLE contacts.notes (
    id_user integer NOT NULL,
    note character varying(250) NOT NULL
);

ALTER TABLE ONLY contacts.notes
    ADD CONSTRAINT fk_id_customer FOREIGN KEY (id_user) REFERENCES contacts.users(id);

