/* eslint-disable camelcase */
/* {
    "id": Math.floor(Math.random() * 1000000) + "",
    "lastSeen": new Date().toISOString().slice(0, 19),
    "grains": null,
    "pillars": null,
    "pkgs": null,
    "lastUpdated_grains": null,
    "lastUpdatedPillars": null,
    "lastUpdatedPkgs": null,
    "conformity": null,
    "conformitySuccess": 0,
    "conformityIncorrect": 0,
    "conformityError": 0,
    "lastUpdatedConformity": null,
} */
export default class Minion {
	static fromObject(data: any): Minion {
		return new Minion(
			data.id,
			data.lastSeen,
			data.grains,
			data.pillars,
			data.pkgs,
			data.lastUpdatedGrains,
			data.lastUpdatedPillars,
			data.lastUpdatedPkgs,
			data.conformity,
			data.conformitySuccess,
			data.conformityIncorrect,
			data.conformityError,
			data.lastUpdatedConformity,
			data.osType,
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
		conformitySuccess: number = 0,
		conformityIncorrect: number = 0,
		conformityError: number = 0,
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
