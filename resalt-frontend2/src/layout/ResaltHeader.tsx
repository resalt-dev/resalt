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
	Alert20Filled,
	Alert20Regular,
	Megaphone20Filled,
	Megaphone20Regular,
	Navigation20Filled,
	Navigation20Regular,
	Person28Filled,
	Question20Filled,
	Question20Regular,
	Settings20Filled,
	bundleIcon,
} from '@fluentui/react-icons';
import { tokens } from '@fluentui/tokens';
import { Signal } from '@preact/signals-react';
import { useEffect } from 'react';
import { useLocation, useNavigate, useParams } from 'react-router-dom';
import ResaltLogo from '../components/ResaltLogo.tsx';
import { getCurrentUser } from '../lib/api.ts';
import { paths } from '../lib/paths.ts';
import User from '../models/User.ts';
import ResaltHeaderSearch from './ResaltHeaderSearch.tsx';

const headerLogoHeight = '20px';
const headerButtonStyles: GriffelStyle = {
	height: '48px',
	width: '48px',
	maxWidth: '48px',
	color: '#ffffff',
	...shorthands.transition('background-color', tokens.durationNormal, tokens.curveEasyEase),
	'&:hover': {
		backgroundColor: tokens.colorNeutralForeground3Hover,
		cursor: 'pointer',
		color: '#ffffff',
	},
	'&:active': {
		backgroundColor: tokens.colorNeutralForeground2Pressed + ' !important',
		color: '#ffffff !important',
	},
};
const useStyles = makeStyles({
	headerGrid: {
		backgroundColor: '#000000',
		color: '#ffffff',
		display: 'grid',
		gridTemplateColumns: `[header-collapse] 48px [header-logo1] 48px [header-logo2] ${
			280 - 48 - 48
		}px [spacer] auto [header-search] 25vw [spacer] auto [header-settings] ${48 * 5}px`,
		gridTemplateRows: '48px',
		alignItems: 'center',
	},
	//
	// Areas
	//
	headerCompose: {
		gridColumnStart: 'header-collapse',
		// backgroundColor: 'rgba(255, 0, 0, 0.5)', // DEBUG
	},
	headerLogoArea1: {
		gridColumnStart: 'header-logo1',
		height: '48px',
		...shorthands.overflow('hidden'),
	},
	headerLogoArea2: {
		gridColumnStart: 'header-logo2',
		height: '48px',
		display: 'flex',
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
	//
	// All "Icons"
	//
	headerLogoImage1: {
		height: 'calc(48px - 16px)',
		display: 'block',
	},
	headerLogoImage2: {
		height: headerLogoHeight,
		...shorthands.padding(tokens.spacingHorizontalL, tokens.spacingHorizontalSNudge),
	},
	headerButton: {
		...headerButtonStyles,
	},
	//
	// Profile Popover
	//
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

const NavigationIcon = bundleIcon(Navigation20Filled, Navigation20Regular);
const MegaphoneIcon = bundleIcon(Megaphone20Filled, Megaphone20Regular);
const AlertIcon = bundleIcon(Alert20Filled, Alert20Regular);
const SettingsIcon = Settings20Filled;
const QuestionIcon = bundleIcon(Question20Filled, Question20Regular);
const PersonIcon = Person28Filled;

const popupUser = new Signal<boolean>(false);

export default function ResaltHeader(props: { currentUser: Signal<User | null> }) {
	console.log('render:ResaltHeader');
	// Navigation
	const location = useLocation();
	const navigate = useNavigate();
	const params = useParams();
	// Styles
	const styles = useStyles();

	useEffect(() => {
		console.log('aaa');
		if (props.currentUser.value !== null) {
			return;
		}
		const from = location.pathname + location.search;
		const to = paths.login.path + '?redirect=' + encodeURIComponent(from);

		getCurrentUser()
			.then((user: User) => {
				console.log('Got current user', user);
				props.currentUser.value = user;
			})
			.catch((err) => {
				console.error('Failed to get current user', err);
				if (location.pathname === paths.login.path) {
					return;
				}
				console.log('Redirecting to', to);
				navigate(to, { replace: true });
			});
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [location, params]);

	return (
		<div className={mergeClasses('m-0', styles.headerGrid)}>
			<div className={styles.headerCompose}>
				<Button
					appearance="transparent"
					shape="square"
					icon={<NavigationIcon />}
					onClick={() => console.log('header:icon:nav')}
					className={styles.headerButton}
				/>
			</div>
			<div className={styles.headerLogoArea1}>
				<Button
					appearance="transparent"
					shape="square"
					icon={
						<img src="/resalt.png" className={styles.headerLogoImage1} alt="Resalt" />
					}
					onClick={() => console.log('header:icon:nav')}
					className={styles.headerButton}
				/>
			</div>
			<div className={styles.headerLogoArea2}>
				<ResaltLogo className={styles.headerLogoImage2} />
			</div>
			<div className={styles.headerSearch}>
				<ResaltHeaderSearch />
			</div>
			<div className={styles.headerSettings}>
				<Button
					appearance="transparent"
					shape="square"
					icon={<MegaphoneIcon />}
					onClick={() => console.log('header:icon:nav')}
					className={styles.headerButton}
				/>
				<Button
					appearance="transparent"
					shape="square"
					icon={<AlertIcon />}
					onClick={() => console.log('header:icon:alert')}
					className={styles.headerButton}
				/>
				<Button
					appearance="transparent"
					shape="square"
					icon={<SettingsIcon />}
					onClick={() => console.log('header:icon:settings')}
					className={styles.headerButton}
				/>
				<Button
					appearance="transparent"
					shape="square"
					icon={<QuestionIcon />}
					onClick={() => console.log('header:icon:help')}
					className={styles.headerButton}
				/>
				<Popover
					open={popupUser.value}
					onOpenChange={() => (popupUser.value = !popupUser.value)}
				>
					<PopoverTrigger>
						<Button
							appearance="transparent"
							shape="square"
							size="large"
							icon={<PersonIcon />}
							onClick={() => console.log('header:icon:user')}
							className={styles.headerButton}
						/>
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
									popupUser.value = false;
								}}
							>
								Sign out
							</Button>
						</div>
					</PopoverSurface>
				</Popover>
			</div>
		</div>
	);
}
