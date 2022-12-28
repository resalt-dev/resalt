// Used by Tabs.svelte
export default class TabPage {
	key: string;

	label: string;

	path: string;

	component: any;

	data?: any;

	hidden?: boolean;
}
