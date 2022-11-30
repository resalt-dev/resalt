// eslint-disable-next-line import/prefer-default-export, no-shadow
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

// eslint-disable-next-line no-redeclare
export namespace RunClientType {
    export function isAsync(client: RunClientType): boolean {
        return client.endsWith('_async');
    }
}

export default RunClientType;
