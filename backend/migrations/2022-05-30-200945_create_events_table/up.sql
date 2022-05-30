CREATE TABLE `events` (
    `id` varchar(50) NOT NULL,
    `timestamp` timestamp(6) NOT NULL,
    `tag` varchar(255) NOT NULL,
    `data` mediumtext NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb3;