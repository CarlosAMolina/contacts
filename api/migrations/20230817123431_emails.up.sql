-- Add up migration script here
CREATE TABLE contacts.emails (
    id_user integer NOT NULL,
    email character varying(50) NOT NULL
);

ALTER TABLE ONLY contacts.emails
    ADD CONSTRAINT fk_id_customer FOREIGN KEY (id_user) REFERENCES contacts.users(id);

