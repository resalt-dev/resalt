import { Text, mergeClasses } from '@fluentui/react-components';
import { CSSProperties } from 'react';
import { useTerminalStyles } from './TerminalCard';

export function TerminalArea(props: {
	id?: string;
	children?: React.ReactNode;
	className?: string;
	style?: CSSProperties;
}) {
	const styles = useTerminalStyles();
	return (
		<div
			id={props.id}
			className={mergeClasses(styles.card, props.className)}
			style={props.style}
		>
			<Text font="monospace" className={styles.cardBody}>
				{props.children}
			</Text>
		</div>
	);
}
