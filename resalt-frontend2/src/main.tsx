import {
	FluentProvider,
	makeStyles,
	mergeClasses,
	shorthands,
	webLightTheme,
} from '@fluentui/react-components';
import { tokens } from '@fluentui/tokens';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { Outlet, RouterProvider, createBrowserRouter } from 'react-router-dom';
import ResaltHeader from './layout/ResaltHeader';
import ResaltSidebar from './layout/ResaltSidebar';
import './lib/fluentui.css';
import paths from './lib/paths';

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

const router = createBrowserRouter([
	{
		path: '/',
		element: <RootLayout />,
		children: Object.entries(paths).map(([_name, path]) => ({
			path: path.path,
			element: React.createElement(path.element as () => JSX.Element, null, null),
		})),
		// children: [
		// 	{
		// 		path: '/',
		// 		element: <div>Home</div>,
		// 	},
		// 	{
		// 		path: '/about',
		// 		element: <div>About</div>,
		// 	},
		// ],
	},
]);

// const currentUser = signal<string | null>(null);

function RootLayout() {
	const styles = useStyles();

	return (
		<FluentProvider theme={webLightTheme} className={styles.fluentProvider}>
			<ResaltHeader />
			<div className={mergeClasses(styles.bodyGrid, 'm-0')}>
				<div className={styles.bodySidebar}>
					<ResaltSidebar />
				</div>
				<div id="mainArea" className={styles.mainArea}>
					<Outlet />
				</div>
			</div>
		</FluentProvider>
	);
}

ReactDOM.createRoot(document.getElementById('root')!).render(
	<React.StrictMode>
		<RouterProvider router={router} />
	</React.StrictMode>,
);
