import { RunClientType } from './RunClientType';

export default class RunResult {
    client: RunClientType;

    targetType: string;

    target: string;

    fun: string;

    arg: string[];

    kwarg: Map<string, string>;

    batchSize: string;

    constructor(
        client: RunClientType,
        targetType: string,
        target: string,
        fun: string,
        arg: string[],
        kwarg: Map<string, string>,
        batchSize: string,
    ) {
        this.client = client;
        this.targetType = targetType;
        this.target = target;
        this.fun = fun;
        this.arg = arg;
        this.kwarg = kwarg;
        this.batchSize = batchSize;
    }

    // return as CMD
    toCommandLine(): string {
        if (this.client === RunClientType.WHEEL || this.client === RunClientType.WHEEL_ASYNC) {
            return '# Not possible to generate command-line equivalent for Wheel runtime';
        }
        let result = '';

        // Client
        if (this.client === RunClientType.LOCAL) {
            result += 'salt';
            if (this.targetType === 'glob') {
                // None
            } else if (this.targetType === 'pcre') {
                result += ' -E';
            } else if (this.targetType === 'list') {
                result += ' -L';
            } else if (this.targetType === 'grain') {
                result += ' -G';
            } else if (this.targetType === 'grain_pcre') {
                result += ' -P';
            } else if (this.targetType === 'pillar') {
                result += ' -I';
            } else if (this.targetType === 'pillar_pcre') {
                result += ' -J';
            } else if (this.targetType === 'nodegroup') {
                result += ' -N';
            } else if (this.targetType === 'range') {
                result += ' -R';
            } else if (this.targetType === 'compound') {
                result += ' -C';
            } else if (this.targetType === 'ipcidr') {
                result += ' -S';
            }
            result += ` "${this.target}"`;
        } else if (this.client === RunClientType.RUNNER) {
            result += 'salt-run';
        }

        // Async
        if (RunClientType.isAsync(this.client)) {
            result += ' --async';
        }

        // Batch
        if (this.client === RunClientType.LOCAL_BATCH) {
            result += ` --batch ${this.batchSize}`;
        }

        // Function
        result += ` ${this.fun}`;

        // Args
        if (this.arg.length > 0) {
            result += ` ${this.arg.map((arg) => arg.indexOf(' ') == -1 ? arg : `"${arg}"`).join(' ')}`;
        }

        // KWargs
        if (this.kwarg.size > 0) {
            this.kwarg.forEach((value, key) => {
                result += ` ${key}=${value}`;
            });
        }
        return result;
    }
}
