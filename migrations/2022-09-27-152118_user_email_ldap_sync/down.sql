ALTER TABLE resalt.users DROP COLUMN `email`;
ALTER TABLE resalt.users DROP COLUMN `ldap_sync`;
ALTER TABLE resalt.permission_groups MODIFY COLUMN ldap_sync varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL NULL;
