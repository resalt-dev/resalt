export default class EventsWriteableData {
	// SaltEvent
	id: string;
	timestamp: string;
	tag: string;
	data: string;

	// Added by loop
	jid: string;
	target: string;
	fun: string;
	dataParsed: any;
	dataFormatted: string;
	uniqueIndex: string;
}
