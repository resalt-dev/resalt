CREATE TABLE `permission_groups` (
  `id` varchar(50) NOT NULL,
  `name` varchar(100) NOT NULL,
  `perms` text DEFAULT '[]' NOT NULL,
  `ldap_sync` varchar(50) NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `permission_group_users` (
  `id` varchar(50) NOT NULL,
  `user_id` varchar(50) NOT NULL,
  `group_id` varchar(50) NOT NULL,
  PRIMARY KEY (`id`),
  CONSTRAINT permission_memberships_FK_uid FOREIGN KEY (user_id) REFERENCES resalt.users(id) ON DELETE CASCADE,
  CONSTRAINT permission_memberships_FK_gid FOREIGN KEY (group_id) REFERENCES resalt.permission_groups(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;