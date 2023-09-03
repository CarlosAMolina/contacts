-- Add up migration script here
CREATE TABLE contacts.urls (
    id_user integer NOT NULL,
    url character varying(250) NOT NULL
);

ALTER TABLE ONLY contacts.urls
    ADD CONSTRAINT fk_id_customer FOREIGN KEY (id_user) REFERENCES contacts.users(id);

