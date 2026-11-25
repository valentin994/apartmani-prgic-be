-- Add down migration script here
ALTER TABLE reservations
DROP COLUMN guest_phone_number;

ALTER TABLE reservations
DROP COLUMN note;
