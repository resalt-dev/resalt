-- remove indexes

ALTER TABLE `jobs` DROP INDEX IF EXISTS `jid`;
ALTER TABLE `jobs` DROP INDEX IF EXISTS `timestamp`;

ALTER TABLE `job_returns` DROP INDEX IF EXISTS `jid`;
ALTER TABLE `job_returns` DROP INDEX IF EXISTS `timestamp`;

ALTER TABLE `events` DROP INDEX IF EXISTS `jid`;
ALTER TABLE `events` DROP INDEX IF EXISTS `timestamp`;
