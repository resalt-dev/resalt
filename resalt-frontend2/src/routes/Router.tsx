import {
	FluentProvider,
	Toast,
	ToastBody,
	ToastTitle,
	Toaster,
	useToastController,
	webLightTheme,
} from '@fluentui/react-components';
import { signal } from '@preact/signals-react';
import React, { useEffect } from 'react';
import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import RootLayout from '../layout/ResaltRootLayout';
import '../lib/fluentui.css';
import { paths } from '../lib/paths';
import { ToastController, ToastMessage } from '../lib/toast';
import User from '../models/User';

export function MainRouter() {
	const currentUser = signal<User | null>(null);

	const [toasts, setToasts] = React.useState<ToastMessage[]>([]);
	const toastController = new ToastController(setToasts);

	const router = createBrowserRouter([
		{
			path: '/',
			element: <RootLayout currentUser={currentUser} />,
			// eslint-disable-next-line @typescript-eslint/no-unused-vars
			children: Object.entries(paths).map(([_name, path]) => ({
				path: path.path,
				element: React.createElement(
					path.element as () => JSX.Element,
					{ currentUser, toastController },
					null,
				),
			})),
		},
	]);

	// dispatch toasts
	const { dispatchToast } = useToastController();
	useEffect(() => {
		for (let toast of toasts) {
			dispatchToast(
				<Toast>
					<ToastTitle>{toast.title}</ToastTitle>
					<ToastBody>{toast.body}</ToastBody>
				</Toast>,
				{ intent: toast.intent },
			);
		}
	});

	return (
		<FluentProvider theme={webLightTheme}>
			<Toaster limit={5} pauseOnHover={true} />
			<RouterProvider router={router} />
		</FluentProvider>
	);
}
