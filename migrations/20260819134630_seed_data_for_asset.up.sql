-- Add up migration script here
-- add seed data for table Asset. It is a table that contains id, name, unit_value. Asset for holdings.
INSERT INTO asset (id, name, unit_value) VALUES (1, 'Stock', 80);
INSERT INTO asset (id, name, unit_value) VALUES (2, 'Bond', 100);
INSERT INTO asset (id, name, unit_value) VALUES (3, 'Real Estate', 150);
