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
	Skeleton,
	SkeletonItem,
	ToolbarButton,
	makeStyles,
	shorthands,
	tokens,
	typographyStyles,
} from '@fluentui/react-components';
import {
	AddFilled,
	AddRegular,
	ArrowReplyRegular,
	MoreHorizontal24Filled,
	ShareRegular,
	bundleIcon,
} from '@fluentui/react-icons';
import { signal } from '@preact/signals-react';
import { useEffect } from 'react';
import { createMinionPreset, getMinionPresets } from '../../lib/api';
import { useGlobalStyles } from '../../lib/ui';
import MinionPreset from '../../models/MinionPreset';

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

const emptyPresets = [
	new MinionPreset('e1', '', '[]'),
	new MinionPreset('e2', '', '[]'),
	new MinionPreset('e3', '', '[]'),
	new MinionPreset('e4', '', '[]'),
	new MinionPreset('e5', '', '[]'),
];
const presets = signal<MinionPreset[] | null>(null);

function loadPresets() {
	getMinionPresets()
		.then((v) => {
			console.log('Got presets', presets);
			presets.value = v;
		})
		.catch((err) => {
			console.error('Failed to get presets', err);
		});
}

function newPreset() {
	createMinionPreset('#NewPreset#', []).then(loadPresets).catch(presetError);
}

function presetError(err: any) {
	console.error('Minion Presets Error', err);
}

const AddIcon = bundleIcon(AddFilled, AddRegular);

export default function MinionsRoute() {
	const globalStyles = useGlobalStyles();
	const styles = useStyles();

	useEffect(loadPresets, []);

	return (
		<>
			<div className="fl-grid">
				<div className="fl-span-2">
					<div className={globalStyles.title}>Minions</div>
				</div>
			</div>
			<div className="fl-grid">
				<div className="fl-span-3">
					<Card>
						<CardHeader
							header={<span className={styles.presetListTitle}>Presets</span>}
							action={
								<Menu>
									<MenuTrigger>
										<ToolbarButton
											aria-label="More"
											icon={<MoreHorizontal24Filled />}
										/>
									</MenuTrigger>

									<MenuPopover>
										<MenuList>
											<MenuItem icon={<AddIcon />} onClick={newPreset}>
												New Preset
											</MenuItem>
											<MenuItem>Copy Preset</MenuItem>
											<MenuItem>Save Preset</MenuItem>
											<MenuItem disabled>Delete Presets</MenuItem>
										</MenuList>
									</MenuPopover>
								</Menu>
							}
						/>
						<div>
							{(presets.value ?? emptyPresets).map((preset) => (
								<div key={preset.id} className={styles.presetItem}>
									{preset.name.length === 0 ? (
										<Skeleton>
											<SkeletonItem />
										</Skeleton>
									) : (
										<span>{preset.name}</span>
									)}
									<br />
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
		</>
	);
}
