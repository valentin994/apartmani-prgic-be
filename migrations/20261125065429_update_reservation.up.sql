-- Add up migration script here
ALTER TABLE reservations
ADD COLUMN guest_phone_number TEXT NOT NULL;

ALTER TABLE reservations
ADD COLUMN note TEXT;
