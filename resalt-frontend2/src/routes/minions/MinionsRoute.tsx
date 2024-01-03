import {
	Body1,
	Button,
	Caption1,
	Card,
	CardFooter,
	CardHeader,
	Menu,
	MenuItem,
	MenuList,
	MenuPopover,
	MenuTrigger,
	Toolbar,
	ToolbarButton,
	ToolbarDivider,
	makeStyles,
	shorthands,
	tokens,
	typographyStyles,
} from '@fluentui/react-components';
import {
	ArrowReplyRegular,
	FontDecrease24Regular,
	FontIncrease24Regular,
	MoreHorizontal24Filled,
	ShareRegular,
	TextFont24Regular,
} from '@fluentui/react-icons';
import { useGlobalStyles } from '../../lib/ui';

const useStyles = makeStyles({
	presetList: {},
	presetListTitle: {
		...typographyStyles.subtitle2Stronger,
		...shorthands.padding(tokens.spacingHorizontalS),
	},
	presetItem: {},
	presetItemTitle: {
		...typographyStyles.body2,
	},
});

export default function MinionsRoute() {
	const globalStyles = useGlobalStyles();
	const styles = useStyles();

	const presets = ['ALL MINIONS', 'NO - Oslo', 'SE - Lund', '1', '2', '3', '4', '5', '6'];

	return (
		<div>
			<div className="fl-grid">
				<div className="fl-span-2">
					<div className={globalStyles.title}>Minions</div>
				</div>
			</div>
			<div className="fl-grid">
				<div className="fl-span-3">
					<Card>
						<div>
							<div className={styles.presetListTitle}>Presets</div>
							<Toolbar size="small">
								<ToolbarButton
									appearance="primary"
									icon={<FontIncrease24Regular />}
								/>
								<ToolbarButton icon={<FontDecrease24Regular />} />
								<ToolbarButton icon={<TextFont24Regular />} />
								<ToolbarDivider />
								<Menu>
									<MenuTrigger>
										<ToolbarButton
											aria-label="More"
											icon={<MoreHorizontal24Filled />}
										/>
									</MenuTrigger>

									<MenuPopover>
										<MenuList>
											<MenuItem>New </MenuItem>
											<MenuItem>New Window</MenuItem>
											<MenuItem disabled>Open File</MenuItem>
											<MenuItem>Open Folder</MenuItem>
										</MenuList>
									</MenuPopover>
								</Menu>
							</Toolbar>
							{presets.map((preset) => (
								<div>
									{preset}
									<br />
								</div>
							))}
						</div>
					</Card>
				</div>
				<div className="fl-span-9">
					<Card>
						<CardHeader
							header={
								<Body1>
									<b>Elvia Atkins</b> mentioned you
								</Body1>
							}
							description={<Caption1>5h ago · About us - Overview</Caption1>}
						/>

						<p>Hi lorum ipsum dolor etat</p>

						<CardFooter>
							<Button icon={<ArrowReplyRegular fontSize={16} />}>Reply</Button>
							<Button icon={<ShareRegular fontSize={16} />}>Share</Button>
						</CardFooter>
					</Card>
				</div>
			</div>
		</div>
	);
}
