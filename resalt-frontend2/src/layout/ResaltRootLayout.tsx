import { makeStyles, mergeClasses, shorthands } from '@fluentui/react-components';
import { tokens } from '@fluentui/tokens';
import { useState } from 'react';
import { Outlet, useLocation } from 'react-router-dom';
import ResaltSidebar from '../layout/ResaltSidebar';
import '../lib/fluentui.css';
import { paths } from '../lib/paths';
import User from '../models/User';
import ResaltHeader from './header/ResaltHeader';

const useStyles = makeStyles({
	rootLayout: {},
	bodyGrid: {
		display: 'grid',
		gridTemplateColumns: '[sidebar] 280px [main-area] auto',
		gridTemplateRows: 'auto',
		...shorthands.transition(
			'grid-template-columns',
			tokens.durationNormal,
			tokens.curveEasyEase,
		),
		"&[data-sidebar-collapsed='true']": {
			gridTemplateColumns: '[sidebar] 48px [main-area] auto',
		},
	},
	bodyGridSidebarHidden: {
		gridTemplateColumns: '[main-area] auto',
	},
	bodySidebar: {
		gridColumnStart: 'sidebar',
		height: 'calc(100vh - 48px)',
		backgroundColor: tokens.colorNeutralBackground4,
	},
	mainArea: {
		gridColumnStart: 'main-area',
		height: 'calc(100vh - 48px)',
		...shorthands.overflow('auto'),
		backgroundColor: tokens.colorNeutralBackground3,
		// backgroundColor: 'rgba(255, 0, 0, 0.2)', // DEBUG
	},
});

export default function RootLayout(props: {
	currentUser: User | null;
	setCurrentUser: React.Dispatch<React.SetStateAction<User | null>>;
}) {
	const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
	const styles = useStyles();
	const location = useLocation();

	const isLoginPage = location.pathname.startsWith(paths.login.path);
	return (
		<div className={styles.rootLayout}>
			<ResaltHeader
				currentUser={props.currentUser}
				setCurrentUser={props.setCurrentUser}
				sidebarCollapsed={sidebarCollapsed}
				setSidebarCollapsed={setSidebarCollapsed}
			/>
			<div
				className={mergeClasses(
					styles.bodyGrid,
					isLoginPage ? styles.bodyGridSidebarHidden : '',
				)}
				data-sidebar-collapsed={sidebarCollapsed}
			>
				{isLoginPage ? null : (
					<div className={styles.bodySidebar}>
						<ResaltSidebar collapsed={sidebarCollapsed} />
					</div>
				)}
				<div id="mainArea" className={styles.mainArea}>
					<Outlet />
				</div>
			</div>
		</div>
	);
}
