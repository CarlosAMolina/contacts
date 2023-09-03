-- Add up migration script here
CREATE TABLE contacts.instagram (
    id_user integer NOT NULL,
    handle character varying(50) NOT NULL
);

ALTER TABLE ONLY contacts.instagram
    ADD CONSTRAINT instagram_id_user_fk FOREIGN KEY (id_user) REFERENCES contacts.users(id);

