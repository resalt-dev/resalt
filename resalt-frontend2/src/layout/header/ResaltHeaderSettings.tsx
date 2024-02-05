import {
	Button,
	Dialog,
	DialogActions,
	DialogBody,
	DialogContent,
	DialogSurface,
	DialogTitle,
	DialogTrigger,
	makeStyles,
} from '@fluentui/react-components';
import { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import User from '../../models/User';

const useStyles = makeStyles({});

export function ResaltHeaderSettings(props: {
	children: React.ReactNode;
	currentUser: User | null;
}) {
	const { currentUser, children } = props;
	const [settingsPopupOpen, setSettingsPopupOpen] = useState(false);

	const location = useLocation();
	const navigate = useNavigate();
	const styles = useStyles();

	// Close popup when navigating
	useEffect(() => {
		setSettingsPopupOpen(false);
	}, [location]);

	return (
		<Dialog open={settingsPopupOpen} onOpenChange={() => setSettingsPopupOpen((v) => !v)}>
			<DialogTrigger disableButtonEnhancement>{children as React.ReactElement}</DialogTrigger>
			<DialogSurface>
				<DialogBody>
					<DialogTitle>Settings</DialogTitle>
					<DialogContent>
						Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam
						exercitationem cumque repellendus eaque est dolor eius expedita nulla ullam?
						Tenetur reprehenderit aut voluptatum impedit voluptates in natus iure cumque
						eaque? Hi :D
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
