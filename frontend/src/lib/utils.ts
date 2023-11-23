const QUOTE_GROUPS = /[^\s"']+|(?:"|'){2,}|"(?!")([^"]*)"|'(?!')([^']*)'|"|'/g;

export function quoteSplit(abc: string): string[] {
	const matches: string[] = [];

	let match: RegExpExecArray | null;
	// eslint-disable-next-line no-cond-assign
	while ((match = QUOTE_GROUPS.exec(abc))) {
		if (match[2]) {
			// Single quoted group
			matches.push(match[2]);
		} else if (match[1]) {
			// Double quoted group
			matches.push(match[1]);
		} else if (match[0]) {
			// No quote group present
			matches.push(match[0]);
		}
	}
	return matches;
}

export function formatAsSize(megabytes: unknown) {
	if (typeof megabytes !== 'number') {
		return null;
	}
	if (megabytes < 1024) {
		return `${megabytes} MB`;
	} else {
		return `${(megabytes / 1024).toFixed(2)} GB`;
	}
}
