import {
	Button,
	GriffelStyle,
	Popover,
	PopoverSurface,
	PopoverTrigger,
	SkeletonItem,
	makeStyles,
	mergeClasses,
	shorthands,
} from '@fluentui/react-components';
import {
	Alert20Regular,
	Megaphone20Regular,
	Navigation20Regular,
	Person28Filled,
	Question20Filled,
	Settings20Regular,
} from '@fluentui/react-icons';
import { tokens } from '@fluentui/tokens';
import { Signal } from '@preact/signals-react';
import { useNavigate } from 'react-router-dom';
import ResaltLogo from '../components/ResaltLogo.tsx';
import paths from '../lib/paths.ts';
import User from '../models/User.ts';
import ResaltHeaderSearch from './ResaltHeaderSearch.tsx';

const headerLogoHeight = '20px';
const iconStyles: GriffelStyle = {
	height: '48px',
	width: '48px',
	display: 'flex',
	alignItems: 'center',
	justifyContent: 'center',
	...shorthands.transition('background-color', tokens.durationNormal, tokens.curveEasyEase),
	'&:hover': {
		backgroundColor: tokens.colorNeutralForeground3Hover,
		cursor: 'pointer',
	},
};
const useStyles = makeStyles({
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
	headerProfilePopover: {
		...shorthands.padding('0'),
		...shorthands.borderRadius('0'),
		minWidth: '325px',
	},
	headerProfilePopoverGrid: {
		// alignItems: 'center',
	},
	headerProfilePopoverUsername: {
		...shorthands.padding(tokens.spacingHorizontalM),
	},
	headerProfilePopoverLogout: {
		width: '100%',
	},
});

export default function ResaltHeader(props: { currentUser: Signal<User | null> }) {
	console.log('render:ResaltHeader');
	const styles = useStyles();
	const navigate = useNavigate();

	return (
		<div className={mergeClasses('m-0', styles.headerGrid)}>
			<div className={styles.headerCollapse}>
				<Navigation20Regular />
			</div>
			<div className={styles.headerLogo}>
				<ResaltLogo className="mx-auto" height={headerLogoHeight} />
			</div>
			<div className={styles.headerSearch}>
				<ResaltHeaderSearch />
			</div>
			<div className={styles.headerSettings}>
				<div className={styles.headerSettingItem}>
					<Megaphone20Regular />
				</div>
				<div className={styles.headerSettingItem}>
					<Alert20Regular />
				</div>
				<div className={styles.headerSettingItem}>
					<Settings20Regular />
				</div>
				<div className={styles.headerSettingItem}>
					<Question20Filled />
				</div>
				<Popover {...props}>
					<PopoverTrigger>
						<div className={styles.headerSettingItem}>
							<Person28Filled />
						</div>
					</PopoverTrigger>

					<PopoverSurface tabIndex={-1} className={styles.headerProfilePopover}>
						<div
							className={mergeClasses(
								'fl-grid',
								'm-0',
								styles.headerProfilePopoverGrid,
							)}
						>
							<div
								className={mergeClasses(
									'fl-span-8',
									styles.headerProfilePopoverUsername,
								)}
							>
								{props.currentUser.value === null ? (
									<SkeletonItem />
								) : (
									<span>{props.currentUser.value.username}</span>
								)}
							</div>
							<Button
								appearance="subtle"
								className={mergeClasses(
									'fl-span-4',
									styles.headerProfilePopoverLogout,
								)}
								onClick={() => {
									navigate(paths.logout.path);
								}}
							>
								Logout
							</Button>
						</div>
					</PopoverSurface>
				</Popover>
			</div>
		</div>
	);
}
