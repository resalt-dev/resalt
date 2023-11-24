export default class Minion {
	static fromObject(data: unknown): Minion {
		const {
			id,
			lastSeen,
			grains,
			pillars,
			pkgs,
			lastUpdatedGrains,
			lastUpdatedPillars,
			lastUpdatedPkgs,
			conformity,
			conformitySuccess,
			conformityIncorrect,
			conformityError,
			lastUpdatedConformity,
			osType,
		} = data as Minion;
		return new Minion(
			id,
			lastSeen,
			grains,
			pillars,
			pkgs,
			lastUpdatedGrains,
			lastUpdatedPillars,
			lastUpdatedPkgs,
			conformity,
			conformitySuccess,
			conformityIncorrect,
			conformityError,
			lastUpdatedConformity,
			osType,
		);
	}

	id: string;

	lastSeen: string;

	grains: string | null;

	pillars: string | null;

	pkgs: string | null;

	lastUpdatedGrains: string | null;

	lastUpdatedPillars: string | null;

	lastUpdatedPkgs: string | null;

	conformity: string | null;

	conformitySuccess: number;

	conformityIncorrect: number;

	conformityError: number;

	lastUpdatedConformity: string | null;

	osType: string | null;

	constructor(
		id: string,
		lastSeen: string,
		grains: string | null = null,
		pillars: string | null = null,
		pkgs: string | null = null,
		lastUpdatedGrains: string | null = null,
		lastUpdatedPillars: string | null = null,
		lastUpdatedPkgs: string | null = null,
		conformity: string | null = null,
		conformitySuccess = 0,
		conformityIncorrect = 0,
		conformityError = 0,
		lastUpdatedConformity: string | null = null,
		osType: string | null = null,
	) {
		this.id = id;
		this.lastSeen = lastSeen;
		this.grains = grains;
		this.pillars = pillars;
		this.pkgs = pkgs;
		this.lastUpdatedGrains = lastUpdatedGrains;
		this.lastUpdatedPillars = lastUpdatedPillars;
		this.lastUpdatedPkgs = lastUpdatedPkgs;
		this.conformity = conformity;
		this.conformitySuccess = conformitySuccess;
		this.conformityIncorrect = conformityIncorrect;
		this.conformityError = conformityError;
		this.lastUpdatedConformity = lastUpdatedConformity;
		this.osType = osType;
	}
}
