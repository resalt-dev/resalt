import {
	Button,
	Popover,
	PopoverSurface,
	PopoverTrigger,
	SkeletonItem,
	makeStyles,
	mergeClasses,
	shorthands,
	tokens,
} from '@fluentui/react-components';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { paths } from '../../lib/paths';
import User from '../../models/User';

const useStyles = makeStyles({
	headerProfilePopover: {
		...shorthands.padding('0'),
		...shorthands.borderRadius('0'),
		minWidth: '325px',
	},
	headerProfilePopoverGrid: {
		// alignItems: 'center',
	},
	headerProfilePopoverUsername: {
		...shorthands.padding(tokens.spacingHorizontalM),
	},
	headerProfilePopoverLogout: {
		width: '100%',
	},
});

export function ResaltHeaderUser(props: { children: React.ReactNode; currentUser: User | null }) {
	const { currentUser, children } = props;
	const [userPopupOpen, setUserPopupOpen] = useState(false);

	const navigate = useNavigate();
	const styles = useStyles();

	return (
		<>
			<Popover
				open={userPopupOpen}
				onOpenChange={() => {
					setUserPopupOpen((v) => !v);
				}}
			>
				<PopoverTrigger>{children as React.ReactElement}</PopoverTrigger>

				<PopoverSurface tabIndex={-1} className={styles.headerProfilePopover}>
					<div
						className={mergeClasses('fl-grid', 'm-0', styles.headerProfilePopoverGrid)}
					>
						<div
							className={mergeClasses(
								'fl-span-8',
								styles.headerProfilePopoverUsername,
							)}
						>
							{currentUser === null ? (
								<SkeletonItem />
							) : (
								<span>{currentUser.username}</span>
							)}
						</div>
						<Button
							appearance="subtle"
							className={mergeClasses('fl-span-4', styles.headerProfilePopoverLogout)}
							onClick={() => {
								navigate(paths.logout.path);
								setUserPopupOpen(false);
							}}
						>
							Sign out
						</Button>
					</div>
				</PopoverSurface>
			</Popover>
		</>
	);
}
