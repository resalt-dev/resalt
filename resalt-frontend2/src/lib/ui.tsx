import { makeStyles, shorthands, tokens, typographyStyles } from '@fluentui/react-components';

export const SKEL = '§§§';

export const useGlobalStyles = makeStyles({
	title: {
		...typographyStyles.subtitle1,
		...shorthands.padding(tokens.spacingHorizontalXXL, tokens.spacingHorizontalSNudge),
		alignItems: 'center',
		display: 'flex',
	},
	cardHeaderTitle: {
		...typographyStyles.subtitle2Stronger,
		...shorthands.padding(tokens.spacingHorizontalS),
		alignItems: 'center',
		display: 'flex',
	},
});

export const formatDate = (datetime: string) => {
	// Convert ISO-8601 to YYYY-MM-DD HH:MM:SS
	return datetime.replace('T', ' ').replace(/\.\d+Z$/, '');
};

export const multilineText = (text: string | number | boolean) => {
	const textStr = text.toString();
	return (
		<>
			{textStr.split(/\n|\r\n/).map((segment, index) => (
				<>
					{index > 0 && <br />}
					{segment}
				</>
			))}
		</>
	);
};
