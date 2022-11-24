ALTER TABLE resalt.users ADD email varchar(256) DEFAULT NULL NULL;
ALTER TABLE resalt.users ADD ldap_sync varchar(255) DEFAULT NULL NULL;
ALTER TABLE resalt.permission_groups MODIFY COLUMN ldap_sync varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL NULL;
