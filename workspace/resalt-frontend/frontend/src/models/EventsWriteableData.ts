export default class EventsWriteableData {
	// Added from {SaltEvent} object
	id: string;
	timestamp: string;
	tag: string;
	data: string;

	// Added by loop
	jid: string;
	target: string;
	fun: string;
	dataParsed: unknown;
	dataFormatted: string;
	uniqueIndex: string;

	constructor(
		id: string,
		timestamp: string,
		tag: string,
		data: string,
		jid: string,
		target: string,
		fun: string,
		dataParsed: unknown,
		dataFormatted: string,
		uniqueIndex: string,
	) {
		this.id = id;
		this.timestamp = timestamp;
		this.tag = tag;
		this.data = data;
		this.jid = jid;
		this.target = target;
		this.fun = fun;
		this.dataParsed = dataParsed;
		this.dataFormatted = dataFormatted;
		this.uniqueIndex = uniqueIndex;
	}
}
