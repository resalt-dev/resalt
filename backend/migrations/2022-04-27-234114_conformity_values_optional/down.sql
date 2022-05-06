ALTER TABLE hibike.minions MODIFY COLUMN conformity_success int(11) DEFAULT 0 NOT NULL;
ALTER TABLE hibike.minions MODIFY COLUMN conformity_incorrect int(11) DEFAULT 0 NOT NULL;
ALTER TABLE hibike.minions MODIFY COLUMN conformity_error int(11) DEFAULT 0 NOT NULL;
