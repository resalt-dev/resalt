DROP DATABASE IF EXISTS  `hibike`;
CREATE DATABASE  `hibike`
  DEFAULT CHARACTER SET utf8
  DEFAULT COLLATE utf8_general_ci;

GRANT Alter ON hibike.* TO 'hibike'@'%';
GRANT Create ON hibike.* TO 'hibike'@'%';
GRANT Create view ON hibike.* TO 'hibike'@'%';
GRANT Delete ON hibike.* TO 'hibike'@'%';
GRANT Delete history ON hibike.* TO 'hibike'@'%';
GRANT Drop ON hibike.* TO 'hibike'@'%';
GRANT Grant option ON hibike.* TO 'hibike'@'%';
GRANT Index ON hibike.* TO 'hibike'@'%';
GRANT Insert ON hibike.* TO 'hibike'@'%';
GRANT References ON hibike.* TO 'hibike'@'%';
GRANT Select ON hibike.* TO 'hibike'@'%';
GRANT Show view ON hibike.* TO 'hibike'@'%';
GRANT Trigger ON hibike.* TO 'hibike'@'%';
GRANT Update ON hibike.* TO 'hibike'@'%';


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