CREATE TABLE `minion_presets` (
  `id` varchar(50) NOT NULL,
  `name` varchar(100) NOT NULL,
  `filter` text NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;
