import { signal } from '@preact/signals-react';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import RootLayout from './layout/ResaltRootLayout';
import './lib/fluentui.css';
import { paths } from './lib/paths';
import User from './models/User';

const currentUser = signal<User | null>(null);

const router = createBrowserRouter([
	{
		path: '/',
		element: <RootLayout currentUser={currentUser} />,
		// eslint-disable-next-line @typescript-eslint/no-unused-vars
		children: Object.entries(paths).map(([_name, path]) => ({
			path: path.path,
			element: React.createElement(path.element as () => JSX.Element, { currentUser }, null),
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

ReactDOM.createRoot(document.getElementById('root')!).render(
	<React.StrictMode>
		<RouterProvider router={router} />
	</React.StrictMode>,
);
