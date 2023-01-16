export default class MinionPreset {
	static fromObject(data: any): MinionPreset {
		return new MinionPreset(data.id, data.name, data.filter);
	}

	id: string;

	name: string;

	filter: string;

	constructor(id: string, name: string, filter: string) {
		this.id = id;
		this.name = name;
		this.filter = filter;
	}
}
