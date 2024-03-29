import {
	Button,
	Dialog,
	DialogActions,
	DialogBody,
	DialogContent,
	DialogSurface,
	DialogTitle,
	DialogTrigger,
	Input,
	Label,
	makeStyles,
	mergeClasses,
	shorthands,
	tokens,
	typographyStyles,
	useId,
} from '@fluentui/react-components';
import { useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import loginBackground from '../../assets/images/login-background.jpg';
import { login } from '../../lib/api';
import { paths } from '../../lib/paths';

const useStyles = makeStyles({
	loginArea: {
		height: '100vh',
		backgroundImage: `url(${loginBackground})`,
		backgroundSize: 'cover',
	},
	loginFieldsArea: {
		...shorthands.padding(tokens.spacingHorizontalM),
		display: 'grid',
	},
	loginTextArea: {
		...typographyStyles.body1Strong,
		...shorthands.padding(tokens.spacingHorizontalM),
	},
});

export default function LoginRoute() {
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');

	// Navigate
	const navigate = useNavigate();
	const [searchParams] = useSearchParams();
	const redirect = searchParams.get('redirect') || paths.dashboard.path;
	// ID's
	const loginUsername = useId('loginUsername');
	const loginPassword = useId('loginPassword');
	// Styles
	const styles = useStyles();

	function loginSubmit(e: React.FormEvent<HTMLFormElement>) {
		console.log('Login submitted');
		const abort = new AbortController();
		login(username, password, abort.signal)
			.then(() => {
				console.log('Logged in');
				navigate(redirect, { replace: true });
			})
			.catch((err) => {
				console.error('Login failed', err);
			});
		e.preventDefault();
	}

	// Center login area in the middle of the screen.
	return (
		<div className={mergeClasses(styles.loginArea, 'fl-grid', 'm-0')}>
			<div className="fl-span-12">
				<Dialog modalType="alert" open={true} defaultOpen={true}>
					<DialogSurface>
						<form
							onSubmit={(e) => {
								loginSubmit(e);
							}}
						>
							<DialogBody>
								<DialogTitle>Sign in</DialogTitle>
								<DialogContent>
									<div className={styles.loginTextArea}>
										This is a restricted admin area. Unauthorized access is
										prohibited.
									</div>
									<div className={styles.loginFieldsArea}>
										<Label htmlFor={loginUsername}>Username</Label>
										<Input
											id={loginUsername}
											onChange={(e) => {
												setUsername(e.target.value);
											}}
											value={username}
										/>
										<br />
										<Label htmlFor={loginPassword}>Password</Label>
										<Input
											id={loginPassword}
											onChange={(e) => {
												setPassword(e.target.value);
											}}
											value={password}
											type="password"
										/>
									</div>
								</DialogContent>
								<DialogActions>
									<DialogTrigger disableButtonEnhancement>
										<Button appearance="secondary">Back</Button>
									</DialogTrigger>
									<Button type="submit" appearance="primary">
										Sign in
									</Button>
								</DialogActions>
							</DialogBody>
						</form>
					</DialogSurface>
				</Dialog>
			</div>
		</div>
	);
}
