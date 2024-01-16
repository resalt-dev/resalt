import {
	Button,
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
	Settings20Regular,
	bundleIcon,
} from '@fluentui/react-icons';
import { tokens } from '@fluentui/tokens';
import { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import ResaltLogo from '../../components/ResaltLogo.tsx';
import { getCurrentUser } from '../../lib/api.ts';
import { paths } from '../../lib/paths.ts';
import User from '../../models/User.ts';
import ResaltHeaderSearch from './ResaltHeaderSearch.tsx';

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
	//
	// Areas
	//
	headerCollapse: {
		gridColumnStart: 'header-collapse',
		// backgroundColor: 'rgba(255, 0, 0, 0.5)', // DEBUG
		'&>button>span': {
			display: 'block',
			...shorthands.transition('transform', tokens.durationNormal, tokens.curveEasyEase),
		},
		'&[data-collapsed="true"]>button>span': {
			transform: 'rotate(90deg)',
		},
	},
	headerLogoArea: {
		gridColumnStart: 'header-logo',
		height: '48px',
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
	//
	// All "Icons"
	//
	headerButton: {
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
		'&>span': {
			width: '32px',
		},
	},
	//
	// Header Logo
	//
	headerLogoButton: {
		width: '100%',
		maxWidth: '100%',
		'&>span': {
			display: 'block',
			...shorthands.margin('0'),
			height: '32px',
			width: '32px',
		},
	},
	headerLogoImage1: {
		height: '32px',
		width: '32px',
		display: 'block',
		userSelect: 'none',
	},
	headerLogoImage2: {
		display: 'block',
		height: '20px',
		maxHeight: '20px',
		...shorthands.padding('0', tokens.spacingHorizontalM),
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
const SettingsIcon = bundleIcon(Settings20Filled, Settings20Regular);
const QuestionIcon = bundleIcon(Question20Filled, Question20Regular);
const PersonIcon = Person28Filled;

export default function ResaltHeader(props: {
	currentUser: User | null;
	setCurrentUser: React.Dispatch<React.SetStateAction<User | null>>;
	sidebarCollapsed: boolean;
	setSidebarCollapsed: React.Dispatch<React.SetStateAction<boolean>>;
}) {
	const { currentUser, setCurrentUser, sidebarCollapsed, setSidebarCollapsed } = props;
	const [userPopupOpen, setUserPopupOpen] = useState(false);

	// Navigation
	const location = useLocation();
	const navigate = useNavigate();
	// Styles
	const styles = useStyles();

	// Load current user if not already loaded
	useEffect(() => {
		// Don't load if already loaded
		if (currentUser !== null) {
			return;
		}
		// Load current user
		const abort = new AbortController();
		getCurrentUser(abort.signal)
			.then((user: User) => {
				console.log('Got current user', user);
				setCurrentUser(user);
			})
			.catch((err) => {
				console.error('Failed to get current user', err);
				if (location.pathname === paths.login.getPath()) {
					return;
				}

				// Don't load if on login page
				const from = location.pathname + location.search;
				const to = paths.login.getPath() + '?redirect=' + encodeURIComponent(from);

				console.log('Redirecting to', to);
				navigate(to, { replace: true });
			});
		return () => {
			abort.abort();
		};
	}, [currentUser, location, navigate, setCurrentUser]);

	return (
		<div className={mergeClasses('m-0', styles.headerGrid)}>
			<div className={styles.headerCollapse} data-collapsed={sidebarCollapsed}>
				<Button
					appearance="transparent"
					shape="square"
					icon={<NavigationIcon />}
					onClick={() => {
						setSidebarCollapsed((v) => !v);
					}}
					className={styles.headerButton}
				/>
			</div>
			<div className={styles.headerLogoArea}>
				<Button
					appearance="transparent"
					shape="square"
					icon={
						<img
							src="/resalt.png"
							className={styles.headerLogoImage1}
							alt="Resalt"
							width="32px"
						/>
					}
					onClick={() => {
						navigate(paths.dashboard.getPath());
					}}
					className={mergeClasses(styles.headerButton, styles.headerLogoButton)}
				>
					<ResaltLogo className={styles.headerLogoImage2} />
				</Button>
			</div>
			<div className={styles.headerSearch}>
				<ResaltHeaderSearch />
			</div>
			<div className={styles.headerSettings}>
				<Button
					appearance="transparent"
					shape="square"
					icon={<MegaphoneIcon />}
					// eslint-disable-next-line @typescript-eslint/no-confusing-void-expression
					onClick={() => console.log('header:icon:nav')}
					className={styles.headerButton}
				/>
				<Button
					appearance="transparent"
					shape="square"
					icon={<AlertIcon />}
					// eslint-disable-next-line @typescript-eslint/no-confusing-void-expression
					onClick={() => console.log('header:icon:alert')}
					className={styles.headerButton}
				/>
				<Button
					appearance="transparent"
					shape="square"
					icon={<SettingsIcon />}
					// eslint-disable-next-line @typescript-eslint/no-confusing-void-expression
					onClick={() => console.log('header:icon:settings')}
					className={styles.headerButton}
				/>
				<Button
					appearance="transparent"
					shape="square"
					icon={<QuestionIcon />}
					// eslint-disable-next-line @typescript-eslint/no-confusing-void-expression
					onClick={() => console.log('header:icon:help')}
					className={styles.headerButton}
				/>
				<Popover
					open={userPopupOpen}
					onOpenChange={() => {
						setUserPopupOpen((v) => !v);
					}}
				>
					<PopoverTrigger>
						<Button
							appearance="transparent"
							shape="square"
							size="large"
							icon={<PersonIcon />}
							// eslint-disable-next-line @typescript-eslint/no-confusing-void-expression
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
								{currentUser === null ? (
									<SkeletonItem />
								) : (
									<span>{currentUser.username}</span>
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
									setUserPopupOpen(false);
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
