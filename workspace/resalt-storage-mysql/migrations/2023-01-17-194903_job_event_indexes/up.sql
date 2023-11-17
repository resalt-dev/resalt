-- create indexes

ALTER TABLE `jobs` ADD INDEX IF NOT EXISTS `jid` (`jid`);
ALTER TABLE `jobs` ADD INDEX IF NOT EXISTS `timestamp` (`timestamp`);

ALTER TABLE `job_returns` ADD INDEX IF NOT EXISTS `jid` (`jid`);
ALTER TABLE `job_returns` ADD INDEX IF NOT EXISTS `timestamp` (`timestamp`);

ALTER TABLE `events` ADD INDEX IF NOT EXISTS `tag` (`tag`);
ALTER TABLE `events` ADD INDEX IF NOT EXISTS `timestamp` (`timestamp`);
