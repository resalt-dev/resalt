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
    data_parsed: any;
    data_formatted: string;
    unique_index: string;
};
