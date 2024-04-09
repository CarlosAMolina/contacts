-- Add up migration script here
ALTER TABLE contacts.phones
    ALTER COLUMN description TYPE character varying(255)
;
