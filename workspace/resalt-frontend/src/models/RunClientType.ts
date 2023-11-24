export enum RunClientType {
	// eslint-disable-next-line no-unused-vars
	LOCAL_BATCH = 'local_batch',
	// eslint-disable-next-line no-unused-vars
	LOCAL_ASYNC = 'local_async',
	// eslint-disable-next-line no-unused-vars
	LOCAL = 'local',
	// eslint-disable-next-line no-unused-vars
	RUNNER_ASYNC = 'runner_async',
	// eslint-disable-next-line no-unused-vars
	RUNNER = 'runner',
	// eslint-disable-next-line no-unused-vars
	WHEEL_ASYNC = 'wheel_async',
	// eslint-disable-next-line no-unused-vars
	WHEEL = 'wheel',
}

export function isRCTAsync(client: RunClientType): boolean {
	return client.endsWith('_async');
}

export function isRCTBatch(client: RunClientType): boolean {
	return client.endsWith('_batch');
}

export function getRCTBaseType(client: RunClientType): string {
	if (isRCTAsync(client)) {
		return client.slice(0, -6);
	}
	if (isRCTBatch(client)) {
		return client.slice(0, -6);
	}
	return client;
}
