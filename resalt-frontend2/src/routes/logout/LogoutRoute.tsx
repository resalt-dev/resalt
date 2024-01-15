import { useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { logout } from '../../lib/api';
import { paths } from '../../lib/paths';
import { ToastController } from '../../lib/toast';
import { useGlobalStyles } from '../../lib/ui';
import User from '../../models/User';

export default function LogoutRoute(props: {
	setCurrentUser: React.Dispatch<React.SetStateAction<User | null>>;
	toastController: ToastController;
}) {
	const { setCurrentUser, toastController } = props;
	const [logoutSuccess, setLogoutSuccess] = useState(false);
	const navigate = useNavigate();

	// Log the user out
	useEffect(() => {
		const abort = new AbortController();
		logout(abort.signal)
			.then(() => {
				setCurrentUser(null);
				setLogoutSuccess(true);
				console.log('Logged out.');
			})
			.catch((err: Error) => {
				toastController.error('Error logging out!', err);
			});
		return () => {
			abort.abort();
		};
	}, [setCurrentUser, toastController]);

	// Redirect the user
	const startTime = useRef(Date.now());
	useEffect(() => {
		if (!logoutSuccess) return;
		const timer = setTimeout(
			() => {
				console.log('Redirecting to', paths.login.path);
				navigate(paths.login.path, { replace: true });
			},
			1500 - (Date.now() - startTime.current),
		);
		return () => {
			clearTimeout(timer);
		};
	}, [logoutSuccess, navigate]);

	const globalStyles = useGlobalStyles();
	return (
		<div>
			<div className="fl-grid">
				<div className="fl-span-12">
					<div className={globalStyles.title}>You are being signed out...</div>
					Please wait until you are redirected.
				</div>
			</div>
		</div>
	);
}
