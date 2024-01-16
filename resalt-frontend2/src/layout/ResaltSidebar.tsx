import { Button, Tab, TabList, makeStyles, shorthands } from '@fluentui/react-components';
import { HomeFilled, HomeRegular, bundleIcon } from '@fluentui/react-icons';
import { tokens, typographyStyles } from '@fluentui/tokens';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { paths, sidebar } from '../lib/paths';

const useStyles = makeStyles({
	sidebarGrid: {
		display: 'grid',
		gridTemplateColumns: 'auto',
		gridTemplateRows: 'auto',
	},
	sidebarDashboardArea: {
		...shorthands.padding(
			tokens.spacingHorizontalXL,
			tokens.spacingHorizontalXL,
			tokens.spacingHorizontalS,
		),
		...shorthands.transition('padding', tokens.durationNormal, tokens.curveEasyEase),
		'&[data-collapsed="true"]': {
			...shorthands.padding(
				tokens.spacingHorizontalXL,
				tokens.spacingHorizontalXS,
				tokens.spacingHorizontalS,
			),
		},
		// backgroundColor: 'rgba(255, 255, 0, 0.5)', // DEBUG
	},
	sidebarHeader: {
		...typographyStyles.subtitle2Stronger,

		...shorthands.padding(
			tokens.spacingHorizontalL,
			tokens.spacingHorizontalM,
			tokens.spacingHorizontalS,
		),
		...shorthands.transition([
			['padding', tokens.durationNormal, tokens.curveEasyEase],
			['width', tokens.durationNormal, tokens.curveEasyEase],
		]),
		width: '0',
		whiteSpace: 'nowrap', // when the text is more than one word
		textAlign: 'center',
		"&[data-collapsed='true']": {
			...shorthands.padding(
				tokens.spacingHorizontalL,
				tokens.spacingHorizontalNone,
				tokens.spacingHorizontalS,
			),
			width: '100%',
		},
	},
	sidebarListArea: {
		...shorthands.padding('0', '0', '0', tokens.spacingHorizontalXS),
		rowGap: '20px',
	},
	sidebarList: {
		width: '100%',
	},
	sidebarItem: {
		width: '100%',
		...shorthands.borderRadius(tokens.borderRadiusXLarge),
		...shorthands.transition('background-color', tokens.durationNormal, tokens.curveEasyEase),
		'&:hover': {
			backgroundColor: tokens.colorNeutralBackground4Hover,
			cursor: 'pointer',
		},

		// Target FIRST sub span
		'& > span:first-child': {
			...shorthands.overflow('visible'),
			width: 'auto',
		},
	},
	sidebarItemIcon: {
		...shorthands.margin('0', '0', '0', tokens.spacingHorizontalS),
		...shorthands.transition('margin', tokens.durationNormal, tokens.curveEasyEase),
		"&[data-collapsed='true']": {
			...shorthands.margin('0', '0', '0', '0'),
		},
	},
});

const HomeIcon = bundleIcon(HomeFilled, HomeRegular);

export default function ResaltSidebar(props: { collapsed: boolean }) {
	const { collapsed } = props;
	const styles = useStyles();
	const navigate = useNavigate();

	// Detect current page
	const location = useLocation();
	let currentPath = paths.dashboard;
	for (const section of sidebar) {
		for (const item of section.items) {
			if (location.pathname.startsWith(item.path) && item.path !== '/') {
				currentPath = item;
				break;
			}
		}
	}

	return (
		<div className={styles.sidebarGrid}>
			<div className={styles.sidebarDashboardArea} data-collapsed={collapsed}>
				<Link to={paths.dashboard.path}>
					<Button
						shape="circular"
						appearance="primary"
						size="large"
						icon={<HomeIcon />}
						onClick={() => {
							navigate(paths.dashboard.path);
						}}
					>
						{collapsed ? '' : paths.dashboard.name}
					</Button>
				</Link>
			</div>
			<div className={styles.sidebarListArea}>
				<TabList
					defaultSelectedValue={currentPath.path}
					vertical
					className={styles.sidebarList}
					size="large"
				>
					{sidebar.map((section) => (
						<div key={section.title}>
							<div className={styles.sidebarHeader} data-collapsed={collapsed}>
								{collapsed ? section.shortTitle : section.title}
							</div>
							{section.items.map((item) => (
								<Link key={item.path} to={item.path}>
									<Tab
										key={item.path}
										className={styles.sidebarItem}
										icon={
											<item.Icon
												className={styles.sidebarItemIcon}
												data-collapsed={collapsed}
											/>
										}
										value={item.path}
									>
										{collapsed ? '' : item.name}
									</Tab>
								</Link>
							))}
						</div>
					))}
				</TabList>
			</div>
		</div>
	);
}
