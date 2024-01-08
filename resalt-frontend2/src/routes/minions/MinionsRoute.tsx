import {
	Button,
	Card,
	CardFooter,
	CardHeader,
	DataGrid,
	DataGridBody,
	DataGridCell,
	DataGridRow,
	Menu,
	MenuItem,
	MenuList,
	MenuPopover,
	MenuTrigger,
	TableCellLayout,
	TableColumnDefinition,
	TableRowId,
	ToolbarButton,
	createTableColumn,
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
	VideoSwitchRegular,
	bundleIcon,
} from '@fluentui/react-icons';
import { signal } from '@preact/signals-react';
import { useEffect } from 'react';
import { createMinionPreset, getMinionPresets, getMinions } from '../../lib/api';
import { showError } from '../../lib/error';
import { useGlobalStyles } from '../../lib/ui';
import Filter from '../../models/Filter';
import Minion from '../../models/Minion';
import MinionPreset from '../../models/MinionPreset';

const useStyles = makeStyles({
	aaa: {
		...typographyStyles.subtitle2Stronger,
		...shorthands.padding(tokens.spacingHorizontalS),
	},
	bbb: {},
	ccc: {
		...typographyStyles.body2,
	},
});

//
// Presets
//

function loadPresets() {
	getMinionPresets()
		.then((v) => {
			console.log('Got presets', presets);
			presets.value = v;
		})
		.catch((err) => showError('Error loading Presets', err));
}

function newPreset() {
	createMinionPreset('#NewPreset#', [])
		.then(loadPresets)
		.catch((err) => showError('Error creating new Preset', err));
}

function selectPreset(selectedItems: Set<TableRowId>) {
	if (selectedItems.size === 0) {
		selectedPreset.value = null;
		return;
	}

	const id = selectedItems.values().next().value as string;
	const preset = presets.value?.find((p) => p.id === id);
	if (!preset) {
		console.error('Failed to find preset', id);
		return;
	}

	if (selectedPreset.value === preset) {
		console.log('Unselecting preset');
		selectedPreset.value = null; // Unselect the item
	} else {
		console.log('Selecting preset', preset);
		selectedPreset.value = preset;
	}
}

//
// Minions
//

function loadMinions() {
	let filters: Filter[] = [];
	let sort = null; // TODO
	let limit = null; // TODO
	let offset = null; // TODO
	getMinions(filters, sort, limit, offset)
		.then((v) => {
			console.log('Got minions', v);
		})
		.catch((err) => showError('Error loading Minions', err));
}

// Used for Skeleton
const emptyPresets = [
	new MinionPreset('e1', '', '[]'),
	new MinionPreset('e2', '', '[]'),
	new MinionPreset('e3', '', '[]'),
	new MinionPreset('e4', '', '[]'),
	new MinionPreset('e5', '', '[]'),
];

const presets = signal<MinionPreset[] | null>(null);
const selectedPreset = signal<MinionPreset | null>(null);

const AddIcon = bundleIcon(AddFilled, AddRegular);

const presetColumns: TableColumnDefinition<MinionPreset>[] = [
	createTableColumn<MinionPreset>({
		columnId: 'name',
		compare: (a, b) => {
			return a.name.localeCompare(b.name);
		},
		renderHeaderCell: () => {
			return 'Name';
		},
		renderCell: (item) => {
			return <TableCellLayout>{item.name}</TableCellLayout>;
		},
	}),
];
const minionColumns: TableColumnDefinition<Minion>[] = [
	createTableColumn<Minion>({
		columnId: 'id',
		compare: (a, b) => {
			return a.id.localeCompare(b.id);
		},
		renderHeaderCell: () => {
			return 'ID';
		},
		renderCell: (item) => {
			return <TableCellLayout>{item.id}</TableCellLayout>;
		},
	}),
];

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
							header={<span className={globalStyles.cardHeaderTitle}>Presets</span>}
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
						<DataGrid
							items={presets.value ?? emptyPresets}
							columns={presetColumns}
							sortable
							sortState={{ sortColumn: 'name', sortDirection: 'ascending' }}
							selectionMode="single"
							getRowId={(item) => item.id}
							onSelectionChange={(_e, data) => selectPreset(data.selectedItems)}
							focusMode="composite"
							size="small"
							subtleSelection={true}
							selectedItems={selectedPreset.value ? [selectedPreset.value.id] : []}
						>
							<DataGridBody<MinionPreset>>
								{({ item, rowId }) => (
									<DataGridRow<MinionPreset>
										key={rowId}
										selectionCell={{
											checkboxIndicator: { 'aria-label': 'Select row' },
										}}
									>
										{({ renderCell }) => (
											<DataGridCell>{renderCell(item)}</DataGridCell>
										)}
									</DataGridRow>
								)}
							</DataGridBody>
						</DataGrid>
					</Card>
				</div>
				<div className="fl-span-9">
					<Card>
						<CardHeader
							header={<span className={globalStyles.cardHeaderTitle}>Filters</span>}
							action={<ToolbarButton icon={<VideoSwitchRegular />} />}
						/>

						<p>Hi lorum ipsum dolor etat</p>

						<CardFooter>
							<Button icon={<ArrowReplyRegular fontSize={16} />}>Reply</Button>
							<Button icon={<ShareRegular fontSize={16} />}>Share</Button>
						</CardFooter>
					</Card>
					<br />
					<Card>
						<CardHeader
							header={<span className={globalStyles.cardHeaderTitle}>Minions</span>}
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
