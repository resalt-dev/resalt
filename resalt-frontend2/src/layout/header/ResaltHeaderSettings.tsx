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
	useId,
} from '@fluentui/react-components';
import { useEffect, useState } from 'react';
import User from '../../models/User';

export function ResaltHeaderSettings(props: {
	children: React.ReactNode;
	currentUser: User | null;
}) {
	const { currentUser, children } = props;
	const [settingsPopupOpen, setSettingsPopupOpen] = useState(false);

	// Close popup when navigating
	useEffect(() => {
		setSettingsPopupOpen(false);
	}, [location]);

	console.log('currentUser', currentUser);

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
									onChange={(_e, data) =>
										(currentUser.preferences.theme = data.checked
											? 'dark'
											: 'light')
									}
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
						<Button appearance="primary">Do Something</Button>
					</DialogActions>
				</DialogBody>
			</DialogSurface>
		</Dialog>
	);
}
