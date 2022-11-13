export type ConformData = {
    __id__: string;
    __run_num__: number;
    __sls__: string;
    changes: any;
    comment: string;
    duration: number;
    name: string;
    result: boolean | null;
    start_time: string;
};
export type Conform = {
    title: string;
    fun: string;
    color: string;
    data: ConformData;
};
export type ConformTreeNode = {
    name: string;
    color: string;
    parent: ConformTreeNode | null;
    subtree: ConformTreeNode[];
    items: Conform[];
};
