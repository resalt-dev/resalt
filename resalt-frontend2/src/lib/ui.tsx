import { makeStyles, shorthands, tokens, typographyStyles } from '@fluentui/react-components';

export const SKEL = '§§§';

export const useGlobalStyles = makeStyles({
	title: {
		...typographyStyles.subtitle1,
		...shorthands.padding(tokens.spacingHorizontalXXL, tokens.spacingHorizontalSNudge),
		alignItems: 'center',
		display: 'flex',
	},
});

export const jsonPalette = {
	base00: 'var(--colorNeutralBackground1)', // Background
	base01: 'var(--colorNeutralBackground5)', // Label Backgrounds
	base05: 'var(--colorNeutralBackgroundInverted)', // Arrows
	base06: 'red', // Keys
	base08: 'var(--colorPalettePeachBorderActive)', // Bools
	base09: 'var(--colorStatusSuccessBorderActive)', // Numbers
	base0A: 'var(--colorPalettePinkBorderActive)', // Symbols
	base0B: 'var(--colorPaletteBlueBorderActive)', // Strings
	base0C: 'var(--colorNeutralBackground5)', // Regex
	base0D: 'var(--colorPalettePeachBorderActive)', // Functions/NULL
	base0E: 'var(--colorPalettePinkBorderActive)', // Undefined
	base0F: 'var(--colorPaletteLightTealBorderActive)', // Dates
};

export const formatDate = (datetime: string) => {
	// Convert ISO-8601 to YYYY-MM-DD HH:MM:SS
	return datetime.replace('T', ' ').replace(/\.\d+Z$/, '');
};

export const multilineText = (text: string | number | boolean) => {
	const textStr = text.toString();
	return (
		<>
			{textStr.split(/\n|\r\n/).map((segment, index) => (
				<span key={index}>
					{index > 0 && <br />}
					{segment}
				</span>
			))}
		</>
	);
};
