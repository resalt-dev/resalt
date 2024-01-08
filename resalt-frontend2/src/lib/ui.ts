import { makeStyles, shorthands, tokens, typographyStyles } from '@fluentui/react-components';

export const SKEL = '§§§';

export const useGlobalStyles = makeStyles({
	title: {
		...typographyStyles.subtitle1,
		...shorthands.padding(tokens.spacingHorizontalXXL, tokens.spacingHorizontalSNudge),
	},
	cardHeaderTitle: {
		...typographyStyles.subtitle2Stronger,
		...shorthands.padding(tokens.spacingHorizontalS),
	},
});

export const formatDate = (datetime: string) => {
	// Convert ISO-8601 to YYYY-MM-DD HH:MM:SS
	return datetime.replace('T', ' ').replace(/\.\d+Z$/, '');
};
