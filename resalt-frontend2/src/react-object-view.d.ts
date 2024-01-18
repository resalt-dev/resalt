declare module 'react-object-view' {
	import { ReactElement } from 'react';

	export interface ObjectViewProps {
		name?: string;
		data: any;
		theme?: any;
		options?: any;
		styles?: any;
		palette?: any;
		customEntry?: any;
		customValue?: any;
		customPreview?: any;
	}

	export const ObjectView: (props: ObjectViewProps) => ReactElement;
	export const customEntry: (type: string, component: any) => void;
	export const customValue: (type: string, component: any) => void;
	export const customPreview: (type: string, component: any) => void;
	export const buildTheme: (theme: any) => any;
	export const defaultPalette: any;
	export const defaultStyles: any;
	export const defaultOptions: any;
}
