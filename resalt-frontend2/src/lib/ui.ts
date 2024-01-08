import { makeStyles, shorthands, tokens, typographyStyles } from '@fluentui/react-components';

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
