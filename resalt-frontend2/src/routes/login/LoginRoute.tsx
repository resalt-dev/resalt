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
import { useNavigate, useParams } from 'react-router-dom';
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
	// Navigate
	const navigate = useNavigate();
	const params = useParams();
	// ID's
	const loginUsername = useId('loginUsername');
	const loginPassword = useId('loginPassword');
	// Styles
	const styles = useStyles();

	function loginSubmit(e: React.FormEvent<HTMLFormElement>) {
		console.log('Login submitted');
		const username = (document.getElementById(loginUsername) as HTMLInputElement).value;
		const password = (document.getElementById(loginPassword) as HTMLInputElement).value;
		const abort = new AbortController();
		login(username, password, abort.signal)
			.then(() => {
				console.log('Logged in');
				const redirect = params.redirect || paths.dashboard.path;
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
						<form onSubmit={(e) => loginSubmit(e)}>
							<DialogBody>
								<DialogTitle>Sign in</DialogTitle>
								<DialogContent>
									<div className={styles.loginTextArea}>
										This is a restricted admin area. Unauthorized access is
										prohibited.
									</div>
									<div className={styles.loginFieldsArea}>
										<Label htmlFor={loginUsername}>Username</Label>
										<Input id={loginUsername} />
										<Label htmlFor={loginPassword}>Password</Label>
										<Input id={loginPassword} type="password" />
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
