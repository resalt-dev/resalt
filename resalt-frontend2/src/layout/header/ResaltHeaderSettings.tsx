import {
	Button,
	Dialog,
	DialogActions,
	DialogBody,
	DialogContent,
	DialogSurface,
	DialogTitle,
	DialogTrigger,
	Label,
	SkeletonItem,
	Switch,
	SwitchOnChangeData,
	useId,
} from '@fluentui/react-components';
import { ChangeEvent, useEffect, useState } from 'react';
import { updateUserPreferences } from '../../lib/api';
import { ToastController } from '../../lib/toast';
import User from '../../models/User';
import UserPreferences from '../../models/UserPreferences';

export function ResaltHeaderSettings(props: {
	children: React.ReactNode;
	currentUser: User | null;
	setCurrentUser: React.Dispatch<React.SetStateAction<User | null>>;
	toastController: ToastController;
}) {
	const { currentUser, setCurrentUser, children, toastController } = props;
	const [settingsPopupOpen, setSettingsPopupOpen] = useState(false);

	// Close popup when navigating
	useEffect(() => {
		setSettingsPopupOpen(false);
	}, [location]);

	function toggleTheme(ev: ChangeEvent<HTMLInputElement>, data: SwitchOnChangeData) {
		const abort = new AbortController();
		const newPrefs: UserPreferences = {
			...currentUser!.preferences,
			theme: data.checked ? 'dark' : 'light',
		};
		updateUserPreferences(currentUser!.id, newPrefs, abort.signal)
			.then(() => {
				const newUser = { ...currentUser!, preferences: newPrefs };
				setCurrentUser(newUser);
				toastController.success(
					'Theme updated',
					`Theme set to ${data.checked ? 'dark' : 'light'}`,
				);
			})
			.catch((e) => {
				toastController.error('Failed to update theme', e);
			});
		ev.preventDefault();
	}

	const themeSetting = useId('themeSetting');
	return (
		<Dialog open={settingsPopupOpen} onOpenChange={() => setSettingsPopupOpen((v) => !v)}>
			<DialogTrigger disableButtonEnhancement>{children as React.ReactElement}</DialogTrigger>
			<DialogSurface>
				<DialogBody>
					<DialogTitle>Settings</DialogTitle>
					<DialogContent>
						{!currentUser ? (
							<SkeletonItem />
						) : (
							<>
								<Label htmlFor={themeSetting}>Dark Theme</Label>
								<Switch
									id={themeSetting}
									onChange={toggleTheme}
									checked={currentUser.preferences.theme === 'dark'}
								/>
								<br />
								<br />
							</>
						)}
					</DialogContent>
					<DialogActions>
						<DialogTrigger disableButtonEnhancement>
							<Button appearance="secondary">Close</Button>
						</DialogTrigger>
					</DialogActions>
				</DialogBody>
			</DialogSurface>
		</Dialog>
	);
}
