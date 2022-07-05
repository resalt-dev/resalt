DROP DATABASE IF EXISTS  `resalt`;
CREATE DATABASE  `resalt`
  DEFAULT CHARACTER SET utf8
  DEFAULT COLLATE utf8_general_ci;

GRANT Alter ON resalt.* TO 'resalt'@'%';
GRANT Create ON resalt.* TO 'resalt'@'%';
GRANT Create view ON resalt.* TO 'resalt'@'%';
GRANT Delete ON resalt.* TO 'resalt'@'%';
GRANT Delete history ON resalt.* TO 'resalt'@'%';
GRANT Drop ON resalt.* TO 'resalt'@'%';
GRANT Grant option ON resalt.* TO 'resalt'@'%';
GRANT Index ON resalt.* TO 'resalt'@'%';
GRANT Insert ON resalt.* TO 'resalt'@'%';
GRANT References ON resalt.* TO 'resalt'@'%';
GRANT Select ON resalt.* TO 'resalt'@'%';
GRANT Show view ON resalt.* TO 'resalt'@'%';
GRANT Trigger ON resalt.* TO 'resalt'@'%';
GRANT Update ON resalt.* TO 'resalt'@'%';


DROP DATABASE IF EXISTS  `salt`;
CREATE DATABASE  `salt`
  DEFAULT CHARACTER SET utf8
  DEFAULT COLLATE utf8_general_ci;

USE `salt`;

--
-- Table structure for table `jids`
--

DROP TABLE IF EXISTS `jids`;
CREATE TABLE `jids` (
  `jid` varchar(255) NOT NULL,
  `load` mediumtext NOT NULL,
  UNIQUE KEY `jid` (`jid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

--
-- Table structure for table `salt_returns`
--

DROP TABLE IF EXISTS `salt_returns`;
CREATE TABLE `salt_returns` (
  `fun` varchar(50) NOT NULL,
  `jid` varchar(255) NOT NULL,
  `return` mediumtext NOT NULL,
  `id` varchar(255) NOT NULL,
  `success` varchar(10) NOT NULL,
  `full_ret` mediumtext NOT NULL,
  `alter_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  KEY `id` (`id`),
  KEY `jid` (`jid`),
  KEY `fun` (`fun`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

--
-- Table structure for table `salt_events`
--

DROP TABLE IF EXISTS `salt_events`;
CREATE TABLE `salt_events` (
`id` BIGINT NOT NULL AUTO_INCREMENT,
`tag` varchar(255) NOT NULL,
`data` mediumtext NOT NULL,
`alter_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
`master_id` varchar(255) NOT NULL,
PRIMARY KEY (`id`),
KEY `tag` (`tag`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;