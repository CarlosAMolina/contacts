-- Add up migration script here
CREATE TABLE contacts.users_categories (
    id_user integer NOT NULL,
    id_category integer NOT NULL
);

ALTER TABLE ONLY contacts.users_categories
    ADD CONSTRAINT users_groups_pk PRIMARY KEY (id_user, id_category);

ALTER TABLE ONLY contacts.users_categories
    ADD CONSTRAINT fk_id_group FOREIGN KEY (id_category) REFERENCES contacts.categories(id);

ALTER TABLE ONLY contacts.users_categories
    ADD CONSTRAINT fk_id_user FOREIGN KEY (id_user) REFERENCES contacts.users(id);
