import ResaltLogo from '../components/ResaltLogo.tsx';
import { tokens } from '@fluentui/tokens';
import {
	Alert20Regular,
	Megaphone20Regular,
	Navigation20Regular,
	Person28Filled,
	Question20Filled,
	Settings20Regular,
} from '@fluentui/react-icons';
import { GriffelStyle, makeStyles, mergeClasses, shorthands } from '@fluentui/react-components';
import ResaltHeaderSearch from './ResaltHeaderSearch.tsx';

const headerLogoHeight = '20px';
const iconStyles: GriffelStyle = {
	height: '48px',
	width: '48px',
	display: 'flex',
	alignItems: 'center',
	justifyContent: 'center',
	...shorthands.transition('background-color', tokens.durationNormal, tokens.curveEasyEase),
	userSelect: 'none',
	'&:hover': {
		backgroundColor: tokens.colorNeutralForeground3Hover,
		cursor: 'pointer',
	},
};
const styles = makeStyles({
	headerGrid: {
		backgroundColor: '#000000',
		color: '#ffffff',
		display: 'grid',
		gridTemplateColumns: `[header-collapse] 48px [header-logo] ${
			280 - 48
		}px [spacer] auto [header-search] 25vw [spacer] auto [header-settings] ${48 * 5}px`,
		gridTemplateRows: '48px',
		alignItems: 'center',
	},
	headerCollapse: {
		gridColumnStart: 'header-collapse',
		...iconStyles,
	},
	headerLogo: {
		gridColumnStart: 'header-logo',
		height: headerLogoHeight,
		...shorthands.overflow('hidden'),
	},
	headerSearch: {
		gridColumnStart: 'header-search',
		// backgroundColor: 'rgba(255, 0, 255, 0.5)', // DEBUG
	},
	headerSettings: {
		gridColumnStart: 'header-settings',
		// Grid
		display: 'grid',
		gridTemplateColumns: 'auto auto auto auto auto auto',
	},
	headerSettingItem: {
		...iconStyles,
	},
});

export default function ResaltHeader() {
	console.log('render:ResaltHeader');
	const classes = styles();

	return (
		<div className={mergeClasses('m-0', classes.headerGrid)}>
			<div className={classes.headerCollapse}>
				<Navigation20Regular />
			</div>
			<div className={classes.headerLogo}>
				<ResaltLogo className="mx-auto" height={headerLogoHeight} />
			</div>
			<div className={classes.headerSearch}>
				<ResaltHeaderSearch />
			</div>
			<div className={classes.headerSettings}>
				<div className={classes.headerSettingItem}>
					<Megaphone20Regular />
				</div>
				<div className={classes.headerSettingItem}>
					<Alert20Regular />
				</div>
				<div className={classes.headerSettingItem}>
					<Settings20Regular />
				</div>
				<div className={classes.headerSettingItem}>
					<Question20Filled />
				</div>
				<div className={classes.headerSettingItem}>
					<Person28Filled />
				</div>
			</div>
		</div>
	);
}
