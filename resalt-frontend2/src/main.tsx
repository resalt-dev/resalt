import React from 'react';
import ReactDOM from 'react-dom/client';
import './lib/fluentui.css';
import {
	FluentProvider,
	makeStyles,
	mergeClasses,
	webLightTheme,
} from '@fluentui/react-components';
import { tokens } from '@fluentui/tokens';
import ResaltHeader from './layout/ResaltHeader';
import ResaltSidebar from './layout/ResaltSidebar';
import { Outlet, RouterProvider, createBrowserRouter } from 'react-router-dom';

const styles = makeStyles({
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
	},
});

const router = createBrowserRouter([
	{
		element: <RootLayout />,
		children: [
			{
				path: '/',
				element: <div>Home</div>,
			},
			{
				path: '/about',
				element: <div>About</div>,
			},
		],
	},
]);

function RootLayout() {
	const classes = styles();
	return (
		<FluentProvider theme={webLightTheme} className={classes.fluentProvider}>
			<ResaltHeader />
			<div className={mergeClasses(classes.bodyGrid, 'm-0')}>
				<div className={classes.bodySidebar}>
					<ResaltSidebar />
				</div>
				<div className={classes.mainArea}>
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
