import {
	FluentProvider,
	Toast,
	ToastBody,
	ToastTitle,
	Toaster,
	useToastController,
	webLightTheme,
} from '@fluentui/react-components';
import React, { useEffect } from 'react';
import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import RootLayout from '../layout/ResaltRootLayout';
import '../lib/fluentui.css';
import { paths } from '../lib/paths';
import { ToastController, ToastMessage } from '../lib/toast';
import User from '../models/User';

export function MainRouter() {
	const [currentUser, setCurrentUser] = React.useState<User | null>(null);
	const [toasts, setToasts] = React.useState<ToastMessage[]>([]);
	const toastController = new ToastController(setToasts);

	const router = createBrowserRouter([
		{
			path: '/',
			element: <RootLayout currentUser={currentUser} setCurrentUser={setCurrentUser} />,
			// eslint-disable-next-line @typescript-eslint/no-unused-vars
			children: Object.entries(paths).map(([_name, path]) => ({
				path: path.path,
				element: React.createElement(
					path.element as () => JSX.Element,
					{ currentUser, setCurrentUser, toastController },
					null,
				),
			})),
		},
	]);

	// dispatch toasts
	const { dispatchToast } = useToastController();
	useEffect(() => {
		for (const toast of toasts) {
			dispatchToast(
				<Toast>
					<ToastTitle>{toast.title}</ToastTitle>
					{toast.body.length > 0 && <ToastBody>{toast.body}</ToastBody>}
				</Toast>,
				{ intent: toast.intent },
			);
		}
		toasts.length = 0;
	});

	return (
		<FluentProvider theme={webLightTheme}>
			<Toaster limit={5} pauseOnHover={true} position="top-end" />
			<RouterProvider router={router} />
		</FluentProvider>
	);
}
