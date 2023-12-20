import { tokens, typographyStyles } from '@fluentui/tokens';
import { Tab, TabList, makeStyles, shorthands } from '@fluentui/react-components';

const styles = makeStyles({
	sidebarGrid: {
		display: 'grid',
		gridTemplateColumns: 'auto',
		gridTemplateRows: 'auto',
	},
	sidebarHeader: {
		...typographyStyles.title1,
		display: 'flex',
		backgroundColor: 'rgba(255, 0, 0, 0.5)', // DEBUG
	},
	sidebarList: {
		display: 'flex',
		alignItems: 'flex-start',
		justifyContent: 'flex-start',
		...shorthands.padding(tokens.spacingHorizontalM, tokens.spacingHorizontalXS),
		rowGap: '20px',

		backgroundColor: 'rgba(0, 255, 0, 0.5)', // DEBUG
	},
	sidebarItem: {
		width: '90%',
	},
});

export default function ResaltSidebar() {
	console.log('render:ResaltSidebar');
	const classes = styles();
	return (
		<div className={classes.sidebarGrid}>
			<div className={classes.sidebarHeader}>Main</div>
			<div className={classes.sidebarList}>
				<TabList defaultSelectedValue="tab2" vertical>
					<Tab value="tab1">First Tab</Tab>
					<Tab value="tab2">Second Tab</Tab>
					<Tab value="tab3">Third Tab</Tab>
					<Tab value="tab4">Fourth Tab</Tab>
				</TabList>
			</div>
		</div>
	);
}
