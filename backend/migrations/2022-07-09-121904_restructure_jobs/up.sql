ALTER TABLE resalt.jobs DROP COLUMN minions;
ALTER TABLE resalt.jobs MODIFY COLUMN `user` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL;
ALTER TABLE resalt.jobs MODIFY COLUMN event_id varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL;
ALTER TABLE resalt.job_returns ADD minion_id varchar(128) NOT NULL;
