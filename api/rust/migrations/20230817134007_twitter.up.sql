-- Add up migration script here
CREATE TABLE contacts.twitter (
    id_user integer NOT NULL,
    handle character varying(50) NOT NULL
);

ALTER TABLE ONLY contacts.twitter
    ADD CONSTRAINT twitter_id_user_fk FOREIGN KEY (id_user) REFERENCES contacts.users(id);

