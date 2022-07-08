CREATE TABLE IF NOT EXISTS `users` (
    `id` varchar(50) NOT NULL,
    `username` varchar(50) NOT NULL,
    `password` varchar(256) DEFAULT NULL,
    `email` varchar(50) NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
CREATE TABLE `authtokens` (
    `id` varchar(50) NOT NULL,
    `user_id` varchar(50) NOT NULL,
    `success` tinyint(1) NOT NULL,
    `timestamp` timestamp NOT NULL DEFAULT current_timestamp(),
    `salt_token` text NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
CREATE TABLE `minions` (
    `id` varchar(128) NOT NULL,
    `last_seen` timestamp NULL DEFAULT NULL,
    `grains` mediumtext DEFAULT NULL,
    `pillars` mediumtext DEFAULT NULL,
    `pkgs` mediumtext DEFAULT NULL,
    `last_updated_grains` timestamp NULL DEFAULT NULL,
    `last_updated_pillars` timestamp NULL DEFAULT NULL,
    `last_updated_pkgs` timestamp NULL DEFAULT NULL,
    `conformity` longtext DEFAULT NULL,
    `conformity_success` int(11) NOT NULL DEFAULT 0,
    `conformity_invalid` int(11) NOT NULL DEFAULT 0,
    `conformity_error` int(11) NOT NULL DEFAULT 0,
    `last_updated_conformity` timestamp NULL DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;