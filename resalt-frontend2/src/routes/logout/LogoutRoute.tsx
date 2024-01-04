import { Signal } from '@preact/signals-react';
import { useNavigate } from 'react-router-dom';
import { logout } from '../../lib/api';
import { paths } from '../../lib/paths';
import { useGlobalStyles } from '../../lib/ui';
import User from '../../models/User';

export default function LogoutRoute(props: { currentUser: Signal<User | null> }) {
	const navigate = useNavigate();

	// Log the user out.
	console.log('Logging out...');
	logout().then(() => {
		setTimeout(() => {
			console.log('Logged out.');
			props.currentUser.value = null;
			// Redirect to login page.
			navigate(paths.login.path, { replace: true });
		}, 1500);
	});

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
