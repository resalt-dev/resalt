import {
	Card,
	CardHeader,
	Text,
	makeStyles,
	mergeClasses,
	shorthands,
	tokens,
	typographyStyles,
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
	cardHeaderSubtitle: {
		textAlign: 'right',
		float: 'right',
		display: 'block',
		paddingTop: tokens.spacingHorizontalXXS,
		...typographyStyles.caption1,
		color: tokens.colorNeutralForeground2,
	},
	cardBody: {
		color: 'white',
		...shorthands.overflow('hidden'),
		"&[data-collapsed='true']": {
			height: '0',
		},
	},
});

export function TerminalCard(props: {
	title: string;
	subtitle?: string;
	children?: React.ReactNode;
	className?: string;
	style?: CSSProperties;
	collapsed: boolean;
	toggleCollapsed: () => void;
}) {
	const styles = useStyles();
	return (
		<Card className={mergeClasses(styles.card, props.className)} style={props.style}>
			<CardHeader
				onClick={props.toggleCollapsed}
				className={mergeClasses(styles.cardHeader, 'mouse-pointer')}
				header={
					<div style={{ width: '100%' }}>
						{props.title}
						<span className={styles.cardHeaderSubtitle}>{props.subtitle}</span>
					</div>
				}
			/>
			<Text font="monospace" className={styles.cardBody} data-collapsed={props.collapsed}>
				{props.children}
			</Text>
		</Card>
	);
}
