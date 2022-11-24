CREATE TABLE `jobs` (
  `id` varchar(50) NOT NULL,
  `timestamp` timestamp(6) NOT NULL DEFAULT current_timestamp(6) ON UPDATE current_timestamp(6),
  `jid` varchar(255) NOT NULL,
  `user` varchar(255) NOT NULL,
  `minions` mediumtext NOT NULL,
  `event_id` varchar(50) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;