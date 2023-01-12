export default class SystemStatus {
	static fromObject(data: any): SystemStatus {
		return new SystemStatus(
			data.salt,
			data.db,
			data.dbAuthTokensActive,
			data.dbAuthTokensTotal,
			data.dbEventsTotal,
			data.dbJobReturnsTotal,
			data.dbJobsTotal,
			data.dbMinionsTotal,
			data.dbPermissionGroupUsersTotal,
			data.dbPermissionGroupsTotal,
			data.dbUsersTotal,
		);
	}

	salt: boolean;
	db: boolean;
	dbAuthTokensTotal: number | null;
	dbAuthTokensActive: number | null;
	dbEventsTotal: number | null;
	dbJobReturnsTotal: number | null;
	dbJobsTotal: number | null;
	dbMinionsTotal: number | null;
	dbPermissionGroupUsersTotal: number | null;
	dbPermissionGroupsTotal: number | null;
	dbUsersTotal: number | null;

	constructor(
		salt: boolean,
		db: boolean,
		dbAuthTokensActive: number | null,
		dbAuthTokensTotal: number | null,
		dbEventsTotal: number | null,
		dbJobReturnsTotal: number | null,
		dbJobsTotal: number | null,
		dbMinionsTotal: number | null,
		dbPermissionGroupUsersTotal: number | null,
		dbPermissionGroupsTotal: number | null,
		dbUsersTotal: number | null,
	) {
		this.salt = salt;
		this.db = db;
		this.dbAuthTokensTotal = dbAuthTokensTotal;
		this.dbAuthTokensActive = dbAuthTokensActive;
		this.dbEventsTotal = dbEventsTotal;
		this.dbJobReturnsTotal = dbJobReturnsTotal;
		this.dbJobsTotal = dbJobsTotal;
		this.dbMinionsTotal = dbMinionsTotal;
		this.dbPermissionGroupUsersTotal = dbPermissionGroupUsersTotal;
		this.dbPermissionGroupsTotal = dbPermissionGroupsTotal;
		this.dbUsersTotal = dbUsersTotal;
	}
}
