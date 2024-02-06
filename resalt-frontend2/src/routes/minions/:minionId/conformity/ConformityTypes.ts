import { tokens } from '@fluentui/tokens';

export enum ConformitySortOrder {
	Incremental = 'inc',
	Decremental = 'dec',
	LongestRuntime = 'runtime',
	BestResult = 'success',
	WorstResult = 'error',
}

export type ConformityData = {
	__id__: string;
	__run_num__: number;
	__sls__: string;
	changes: unknown;
	comment: string;
	duration: number;
	name: string;
	result: boolean | null;
	start_time: string;
};

export function parseConformityData(data: unknown): ConformityData | null {
	if (typeof data !== 'object') {
		return null;
	}
	if (data === null) {
		return null;
	}
	if (Array.isArray(data)) {
		return null;
	}
	if ('__id__' in data === false) return null;
	if (typeof data.__id__ !== 'string') return null;
	if ('__run_num__' in data === false) return null;
	if (typeof data.__run_num__ !== 'number') return null;
	if ('__sls__' in data === false) return null;
	if (typeof data.__sls__ !== 'string') return null;
	if ('changes' in data === false) return null;
	if (typeof data.changes !== 'object') return null;
	if (data.changes === null) return null;
	if ('comment' in data === false) return null;
	if (typeof data.comment !== 'string') return null;
	if ('duration' in data === false) return null;
	if (typeof data.duration !== 'number') return null;
	if ('name' in data === false) return null;
	if (typeof data.name !== 'string') return null;
	if ('result' in data === false) return null;
	if (typeof data.result !== 'boolean' && data.result !== null) return null;
	if ('start_time' in data === false) return null;
	if (typeof data.start_time !== 'string') return null;
	return {
		__id__: data.__id__,
		__run_num__: data.__run_num__,
		__sls__: data.__sls__,
		changes: data.changes,
		comment: data.comment,
		duration: data.duration,
		name: data.name,
		result: data.result,
		start_time: data.start_time,
	};
}

type ConformityStatus = 'success' | 'incorrect' | 'error';
export type Conformity = {
	title: string;
	fun: string;
	status: ConformityStatus;
	data: ConformityData;
};

export function parseConformity(
	conformity: string | null,
	sortOrder?: ConformitySortOrder,
): Conformity[] {
	if (!conformity) return [];
	const parsed = JSON.parse(conformity);
	const entries = Object.entries(parsed);
	const result: Conformity[] = [];
	for (const [key, rawValue] of entries) {
		const value = parseConformityData(rawValue);
		if (!value) {
			console.warn('Failed to parse conformity data', key, rawValue);
			continue;
		}
		let parts = key.split('_|-');
		let conform: Conformity = {
			title: key,
			fun: parts[0] + '.' + parts[parts.length - 1],
			// status should be success/incorrect/error based on true/null/false
			status:
				value.result === true ? 'success' : value.result === false ? 'error' : 'incorrect',
			data: {
				__id__: value.__id__ ?? parts[1] ?? 'UKNOWN ID',
				// eslint-disable-next-line camelcase
				__run_num__: value.__run_num__,
				__sls__: value.__sls__,
				changes: value.changes ?? {},
				comment: value.comment,
				duration: value.duration,
				name: value.name ?? parts[2] ?? 'UKNOWN NAME',
				result: value.result,
				// eslint-disable-next-line camelcase
				start_time: value.start_time,
			} as ConformityData,
		};
		result.push(conform);
	}
	if (sortOrder) {
		result.sort((a, b) => {
			switch (sortOrder) {
				case ConformitySortOrder.Incremental:
					return a.data.__run_num__ - b.data.__run_num__;
				case ConformitySortOrder.Decremental:
					return b.data.__run_num__ - a.data.__run_num__;
				case ConformitySortOrder.LongestRuntime:
					return b.data.duration - a.data.duration;
				case ConformitySortOrder.BestResult:
					return (
						(a.data.result === true ? 1 : a.data.result === false ? 3 : 2) -
						(b.data.result === true ? 1 : b.data.result === false ? 3 : 2)
					);
				case ConformitySortOrder.WorstResult:
					return (
						(b.data.result === true ? 1 : b.data.result === false ? 3 : 2) -
						(a.data.result === true ? 1 : a.data.result === false ? 3 : 2)
					);
				default:
					return 0;
			}
		});
	}
	return result;
}

export type ConformityTreeNode = {
	name: string;
	status: ConformityStatus | '';
	parent: ConformityTreeNode | null;
	subtree: ConformityTreeNode[];
	items: Conformity[];
};

// Reduce above Conformity states to a tree of SLS files
// - a (1)
//   - aa (1)
//   - ab (1)
// - common
//   - init
//     - test (2)
// - editor (1)
//   - vim (7)
function sortSubtreeRecursively(subtree: ConformityTreeNode[]) {
	subtree.sort((a, b) => a.name.localeCompare(b.name));
	subtree.forEach((node) => {
		sortSubtreeRecursively(node.subtree);
	});
}

export function buildConformityTree(conformity: Conformity[]): ConformityTreeNode {
	let conformityTree = conformity.reduce(
		(acc, c) => {
			let parts = c.data.__sls__.split('.');
			let current = acc;
			for (let i = 0; i < parts.length; i++) {
				let part = parts[i];
				let existing = current.subtree.find((e) => e.name === part);
				if (!existing) {
					existing = {
						name: part,
						status: '',
						parent: current,
						subtree: [],
						items: [],
					};
					current.subtree.push(existing);
				}
				current = existing;
			}
			current.items.push(c);
			// Set min status for chain going up
			let parent: ConformityTreeNode | null = current;
			while (parent !== null) {
				if (c.status === 'error') {
					parent.status = 'error';
				} else if (c.status === 'incorrect' && parent.status !== 'error') {
					parent.status = 'incorrect';
				} else if (c.status === 'success' && parent.status === '') {
					parent.status = 'success';
				}
				parent = parent.parent;
			}
			return acc;
		},
		{
			name: '#',
			status: '',
			parent: null,
			subtree: [],
			items: [],
		} as ConformityTreeNode,
	);
	sortSubtreeRecursively(conformityTree.subtree);
	return conformityTree;
}

export function conformityMapFluentColor(status: ConformityStatus | ''): string {
	switch (status) {
		case 'success':
			return 'limegreen';
		case 'incorrect':
			return tokens.colorPaletteYellowBackground3;
		case 'error':
			return 'red';
		default:
			return tokens.colorPalettePurpleBorderActive;
	}
}
