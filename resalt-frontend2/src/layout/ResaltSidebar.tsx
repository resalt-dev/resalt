import { Button, Tab, TabList, makeStyles, shorthands } from '@fluentui/react-components';
import { HomeRegular } from '@fluentui/react-icons';
import { tokens, typographyStyles } from '@fluentui/tokens';
import { Link } from 'react-router-dom';
import { paths, sidebar } from '../lib/paths';

const styles = makeStyles({
	sidebarGrid: {
		display: 'grid',
		gridTemplateColumns: 'auto',
		gridTemplateRows: 'auto',
	},
	sidebarDashboardArea: {
		display: 'flex',
		alignItems: 'flex-start',
		justifyContent: 'flex-start',
		...shorthands.padding(
			tokens.spacingHorizontalXL,
			tokens.spacingHorizontalXL,
			tokens.spacingHorizontalS,
		),
		// backgroundColor: 'rgba(255, 255, 0, 0.5)', // DEBUG
	},
	sidebarDashboardButton: {
		backgroundImage:
			'linear-gradient(135deg, ' +
			tokens.colorBrandBackground +
			' 30%, ' +
			tokens.colorBrandBackgroundSelected +
			' 70%)',
		color: tokens.colorNeutralForegroundInverted,
	},
	sidebarHeader: {
		display: 'flex',
		alignItems: 'flex-start',
		justifyContent: 'flex-start',
		...shorthands.padding(
			tokens.spacingHorizontalL,
			tokens.spacingHorizontalM,
			tokens.spacingHorizontalS,
		),
		...typographyStyles.subtitle2Stronger,
	},
	sidebarListArea: {
		display: 'flex',
		alignItems: 'flex-start',
		justifyContent: 'flex-start',
		...shorthands.padding('0', '0', '0', tokens.spacingHorizontalXS),
		rowGap: '20px',
	},
	sidebarList: {
		width: '100%',
	},
	sidebarItem: {
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
	sidebarItemIconArea: {
		display: 'flex',
		// backgroundColor: 'rgba(0, 255, 255, 0.4)', // DEBUG
	},
	sidebarItemIcon: {
		...shorthands.margin('0', '0', '0', tokens.spacingHorizontalS),
	},
});

export default function ResaltSidebar() {
	console.log('render:ResaltSidebar');
	const classes = styles();
	return (
		<div className={classes.sidebarGrid}>
			<div className={classes.sidebarDashboardArea}>
				<Link to={paths.dashboard.path}>
					<Button
						shape="circular"
						appearance="primary"
						size="large"
						icon={<HomeRegular />}
						className={classes.sidebarDashboardButton}
					>
						{paths.dashboard.name}
					</Button>
				</Link>
			</div>
			<div className={classes.sidebarListArea}>
				<TabList
					defaultSelectedValue={paths.minions.path}
					vertical
					className={classes.sidebarList}
					size="large"
				>
					{sidebar.map((section) => (
						<div key={section.title}>
							<div className={classes.sidebarHeader}>{section.title}</div>
							{section.items.map((item) => (
								<Link key={item.path} to={item.path}>
									<Tab
										key={item.path}
										className={classes.sidebarItem}
										icon={<item.Icon className={classes.sidebarItemIcon} />}
										value={item.path}
									>
										{item.name}
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
