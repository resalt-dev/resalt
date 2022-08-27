export default class RunResult {
    client: string;

    targetType: string;

    target: string;

    fun: string;

    arg: string[];

    kwarg: Map<string, string>;

    batchSize: string;

    timeout: number;

    constructor(
        client: string,
        targetType: string,
        target: string,
        fun: string,
        arg: string[],
        kwarg: Map<string, string>,
        batchSize: string,
        timeout: number,
    ) {
        this.client = client;
        this.targetType = targetType;
        this.target = target;
        this.fun = fun;
        this.arg = arg;
        this.kwarg = kwarg;
        this.batchSize = batchSize;
        this.timeout = timeout;
    }
}
