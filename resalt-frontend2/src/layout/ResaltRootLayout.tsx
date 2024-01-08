import {
	FluentProvider,
	Toaster,
	makeStyles,
	mergeClasses,
	shorthands,
	webLightTheme,
} from '@fluentui/react-components';
import { tokens } from '@fluentui/tokens';
import { Signal } from '@preact/signals-react';
import { Outlet, useLocation } from 'react-router-dom';
import ResaltHeader from '../layout/ResaltHeader';
import ResaltSidebar from '../layout/ResaltSidebar';
import '../lib/fluentui.css';
import { paths } from '../lib/paths';
import User from '../models/User';

const useStyles = makeStyles({
	fluentProvider: {
		backgroundColor: tokens.colorNeutralBackground4,
	},

	//
	// Body
	//
	bodyGrid: {
		display: 'grid',
		gridTemplateColumns: '[sidebar] 280px [main-area] auto',
		gridTemplateRows: 'auto',
	},
	bodyGridSidebarHidden: {
		gridTemplateColumns: '[main-area] auto',
	},
	bodySidebar: {
		gridColumnStart: 'sidebar',
		height: 'calc(100vh - 48px)',
	},
	mainArea: {
		gridColumnStart: 'main-area',
		// backgroundColor: 'rgba(255, 0, 0, 0.2)', // DEBUG
		...shorthands.overflow('auto'),
	},
});

export default function RootLayout(props: { currentUser: Signal<User | null> }) {
	const styles = useStyles();
	const location = useLocation();

	const isLoginPage = location.pathname.startsWith(paths.login.path);
	return (
		<FluentProvider theme={webLightTheme} className={styles.fluentProvider}>
			<Toaster limit={5} />
			<ResaltHeader currentUser={props.currentUser} />
			<div
				className={mergeClasses(
					styles.bodyGrid,
					isLoginPage ? styles.bodyGridSidebarHidden : '',
				)}
			>
				{isLoginPage ? null : (
					<div className={styles.bodySidebar}>
						<ResaltSidebar />
					</div>
				)}
				<div id="mainArea" className={styles.mainArea}>
					<Outlet />
				</div>
			</div>
		</FluentProvider>
	);
}
