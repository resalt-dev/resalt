ALTER TABLE resalt.jobs ADD minions mediumtext NOT NULL;
ALTER TABLE resalt.jobs MODIFY COLUMN `user` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL;
ALTER TABLE resalt.jobs MODIFY COLUMN event_id varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL;
ALTER TABLE resalt.job_returns DROP COLUMN minion_id;
