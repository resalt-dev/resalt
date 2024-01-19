import { Card, CardHeader, Text, makeStyles, shorthands, tokens } from '@fluentui/react-components';

const useStyles = makeStyles({
	card: {
		backgroundColor: 'black',
		...shorthands.padding('0'),
		marginBottom: tokens.spacingVerticalM,
	},
	cardHeader: {
		backgroundColor: tokens.colorNeutralBackground1,
		...shorthands.padding(tokens.spacingHorizontalS),
	},
	cardBody: {
		color: 'white',
	},
});

export function TerminalCard(props: { children?: React.ReactNode }) {
	const styles = useStyles();
	return (
		<Card className={styles.card}>
			<CardHeader className={styles.cardHeader} header="TerminalBox1" />
			<Text font="monospace" className={styles.cardBody}>
				{props.children}
			</Text>
		</Card>
	);
}
