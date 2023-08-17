-- Add up migration script here
CREATE TABLE contacts.phones (
    id_user integer NOT NULL,
    phone bigint NOT NULL,
    description character varying(50)
);

ALTER TABLE ONLY contacts.phones
    ADD CONSTRAINT fk_id_customer FOREIGN KEY (id_user) REFERENCES contacts.users(id);

