CREATE TABLE `job_returns` (
  `id` varchar(50) NOT NULL,
  `timestamp` timestamp(6) NOT NULL DEFAULT current_timestamp(6) ON UPDATE current_timestamp(6),
  `jid` varchar(255) NOT NULL,
  `job_id` varchar(50) NOT NULL,
  `event_id` varchar(50) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;