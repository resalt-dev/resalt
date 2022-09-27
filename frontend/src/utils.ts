const QUOTE_GROUPS = /[^\s"']+|(?:"|'){2,}|"(?!")([^"]*)"|'(?!')([^']*)'|"|'/g;

export function quoteSplit(abc: string): string[] {
    const matches: string[] = [];

    let match;
    // eslint-disable-next-line no-cond-assign
    while ((match = QUOTE_GROUPS.exec(abc))) {
        if (match[2]) {
            // Single quoted group
            matches.push(match[2]);
        } else if (match[1]) {
            // Double quoted group
            matches.push(match[1]);
        } else {
            // No quote group present
            matches.push(match[0]!);
        }
    }
    return matches;
}

export function test(): void {}

// Used by Tabs.svelte
export type NavSubPage = {
    label: string;
    component: any;
    data?: any;
    class?: string;
};
