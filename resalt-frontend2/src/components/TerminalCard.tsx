import {
	Card,
	CardHeader,
	Text,
	makeStyles,
	mergeClasses,
	shorthands,
	tokens,
} from '@fluentui/react-components';
import { CSSProperties } from 'react';

const useStyles = makeStyles({
	card: {
		backgroundColor: 'black',
		...shorthands.padding('0'),
		marginBottom: tokens.spacingVerticalM,
	},
	cardHeader: {
		backgroundColor: tokens.colorNeutralBackground1,
		...shorthands.padding(tokens.spacingHorizontalSNudge, tokens.spacingHorizontalS),
	},
	cardBody: {
		color: 'white',
	},
});

export function TerminalCard(props: {
	title: string;
	children?: React.ReactNode;
	className?: string;
	style?: CSSProperties;
}) {
	const styles = useStyles();
	return (
		<Card className={mergeClasses(styles.card, props.className)} style={props.style}>
			<CardHeader className={styles.cardHeader} header={props.title} />
			<Text font="monospace" className={styles.cardBody}>
				{props.children}
			</Text>
		</Card>
	);
}
