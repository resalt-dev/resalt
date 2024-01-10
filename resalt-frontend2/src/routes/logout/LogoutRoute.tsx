import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { logout } from '../../lib/api';
import { paths } from '../../lib/paths';
import { useGlobalStyles } from '../../lib/ui';
import User from '../../models/User';

export default function LogoutRoute(props: {
	setCurrentUser: React.Dispatch<React.SetStateAction<User | null>>;
}) {
	const [logoutSuccess, setLogoutSuccess] = useState(false);
	const navigate = useNavigate();

	// Log the user out
	useEffect(() => {
		const abort = new AbortController();
		logout(abort.signal).then(() => {
			props.setCurrentUser(null);
			setLogoutSuccess(true);
			console.log('Logged out.');
		});
		return () => abort.abort();
	}, []);

	// Redirect the user
	useEffect(() => {
		if (!logoutSuccess) return;
		const timer = setTimeout(() => {
			console.log('Redirecting to', paths.login.path);
			navigate(paths.login.path, { replace: true });
		}, 1500);
		return () => clearTimeout(timer);
	}, [logoutSuccess]);

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
