import {
	Badge,
	Button,
	Card,
	CardFooter,
	CardHeader,
	DataGrid,
	DataGridBody,
	DataGridCell,
	DataGridRow,
	Input,
	Menu,
	MenuItem,
	MenuList,
	MenuPopover,
	MenuTrigger,
	Select,
	SkeletonItem,
	Table,
	TableBody,
	TableCell,
	TableCellLayout,
	TableColumnDefinition,
	TableColumnId,
	TableHeader,
	TableHeaderCell,
	TableRow,
	TableRowId,
	ToolbarButton,
	createTableColumn,
	makeStyles,
	mergeClasses,
	shorthands,
	tokens,
	typographyStyles,
	useTableFeatures,
	useTableSort,
} from '@fluentui/react-components';
import {
	AddFilled,
	AddRegular,
	AppsFilled,
	AppsRegular,
	ArrowMinimizeFilled,
	ArrowMinimizeRegular,
	ArrowReplyRegular,
	InfoFilled,
	InfoRegular,
	MoreHorizontal24Filled,
	ServerFilled,
	ServerRegular,
	ShareRegular,
	SwipeDownFilled,
	SwipeDownRegular,
	bundleIcon,
} from '@fluentui/react-icons';
import { signal } from '@preact/signals-react';
import { useEffect } from 'react';
import { createMinionPreset, getMinionPresets, getMinions } from '../../lib/api';
import { showError } from '../../lib/error';
import { SKEL, formatDate, useGlobalStyles } from '../../lib/ui';
import Filter from '../../models/Filter';
import { FilterFieldType } from '../../models/FilterFieldType';
import { FilterOperand } from '../../models/FilterOperand';
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
			presetsLoaded.value = true;
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
	if (id.startsWith(SKEL)) {
		console.log('Ignoring skeleton item');
		return;
	}
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
// Filters
//

function addFilter() {
	filters.value = [...filters.value, Filter.newEmpty()];
}

function updateFilter(f: Filter, fieldType: string, newValue: string): void {
	const copy: Filter[] = JSON.parse(JSON.stringify(filters.value)).map(Filter.fromObject);
	const filter = copy.filter((f2) => f2.id === f.id)[0];
	console.log('Updating filter', f, copy);
	if (!filter) {
		console.error('Failed to find filter', f);
		return;
	}
	switch (fieldType) {
		case 'fieldType':
			filter.fieldType = newValue as FilterFieldType;
			filter.field = filter.fieldType === FilterFieldType.OBJECT ? 'id' : '';
			filter.operand = FilterOperand.CONTAINS;
			filter.value = '';
			break;
		case 'field':
			filter.field = newValue;
			if (
				[
					'last_seen',
					'conformity_success',
					'conformity_incorrect',
					'conformity_error',
				].includes(newValue)
			) {
				filter.operand = FilterOperand.GREATER_THAN_OR_EQUAL;
			}
			break;
		case 'operand':
			filter.operand = newValue as FilterOperand;
			break;
		case 'value':
			filter.value = newValue;
			break;
		default:
			console.error('Unknown filter field', fieldType);
			return;
	}
	filters.value = copy;
	loadMinions();
}

function toggleFiltersExpand() {
	filtersExpanded.value = !filtersExpanded.value;
}

//
// Minions
//

let loadMinionsTaskID: string | null = null;
function loadMinions() {
	let sort = null; // TODO
	let limit = null; // TODO
	let offset = null; // TODO
	let taskID = Math.random().toString(36).substring(2);
	loadMinionsTaskID = taskID;
	getMinions(filters.value, sort, limit, offset)
		.then((v) => {
			console.log('Got minions', v);
			if (loadMinionsTaskID !== taskID) {
				console.log('Ignoring minions response, newer request in progress');
				return;
			}
			minions.value = v;
			minionsLoaded.value = true;
		})
		.catch((err) => showError('Error loading Minions', err));
}

// Used for Skeleton
const emptyPresets = [
	new MinionPreset(`${SKEL}1`, '', '[]'),
	new MinionPreset(`${SKEL}2`, '', '[]'),
	new MinionPreset(`${SKEL}3`, '', '[]'),
	new MinionPreset(`${SKEL}4`, '', '[]'),
	new MinionPreset(`${SKEL}5`, '', '[]'),
];
const emptyMinions = [
	new Minion(`${SKEL}1`, ''),
	new Minion(`${SKEL}2`, ''),
	new Minion(`${SKEL}3`, ''),
	new Minion(`${SKEL}4`, ''),
	new Minion(`${SKEL}5`, ''),
];

const presets = signal<MinionPreset[] | null>(null);
const presetsLoaded = signal(false);
const selectedPreset = signal<MinionPreset | null>(null);
const minions = signal<Minion[] | null>(null);
const minionsLoaded = signal(false);
const filters = signal<Filter[]>([]);
const filtersExpanded = signal(true);

const AddIcon = bundleIcon(AddFilled, AddRegular);
const CollapseIcon = bundleIcon(ArrowMinimizeFilled, ArrowMinimizeRegular);
const ExpandIcon = bundleIcon(SwipeDownFilled, SwipeDownRegular);
const FilterMinionIcon = bundleIcon(ServerFilled, ServerRegular);
const FilterGrainIcon = bundleIcon(InfoFilled, InfoRegular);
const FilterPackageIcon = bundleIcon(AppsFilled, AppsRegular);

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
	useEffect(loadMinions, []);

	// Minions table
	const {
		getRows,
		sort: { getSortDirection, toggleColumnSort, sort },
	} = useTableFeatures(
		{
			columns: minionColumns,
			items: minions.value ?? (minionsLoaded.value ? [] : emptyMinions),
		},
		[
			useTableSort({
				defaultSortState: { sortColumn: 'id', sortDirection: 'ascending' },
			}),
		],
	);
	const headerSortProps = (columnId: TableColumnId) => ({
		onClick: (e: React.MouseEvent) => {
			toggleColumnSort(e, columnId);
		},
		sortDirection: getSortDirection(columnId),
	});
	const rows = sort(getRows());

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
							items={presets.value ?? (presetsLoaded.value ? [] : emptyPresets)}
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
								{({ item }) => (
									<DataGridRow<MinionPreset>
										key={item.id}
										selectionCell={{
											checkboxIndicator: { 'aria-label': 'Select row' },
										}}
									>
										{({ renderCell }) => (
											<DataGridCell>
												{item.id.startsWith(SKEL) ? (
													<SkeletonItem />
												) : (
													renderCell(item)
												)}
											</DataGridCell>
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
							className="mouse-pointer"
							onClick={toggleFiltersExpand}
							header={
								<>
									<span
										className={mergeClasses(
											globalStyles.cardHeaderTitle,
											'mouse-pointer',
										)}
									>
										Search
									</span>
									{filters.value.filter((f) => f.isValid()).length > 0 && (
										<Badge
											color="brand"
											size="large"
											shape="rounded"
											className="mx-s my-snudge"
										>
											Filters active!
										</Badge>
									)}
								</>
							}
							action={
								<ToolbarButton
									icon={filtersExpanded.value ? <CollapseIcon /> : <ExpandIcon />}
								/>
							}
						/>

						{(filtersExpanded.value ? filters.value : []).map((f) => (
							<div key={f.id} className="fl-grid-small mx-0">
								<Select
									className="fl-span-2"
									onChange={(_e, data) =>
										updateFilter(f, 'fieldType', data.value)
									}
									value={f.fieldType}
								>
									<option value={FilterFieldType.NONE}>None</option>
									<option value={FilterFieldType.OBJECT}>Minion</option>
									<option value={FilterFieldType.GRAIN}>Grain</option>
									<option value={FilterFieldType.PACKAGE}>Package</option>
								</Select>

								{f.fieldType === FilterFieldType.OBJECT && (
									<Select
										className="fl-span-3"
										onChange={(_e, data) =>
											updateFilter(f, 'field', data.value)
										}
										value={f.field}
									>
										<option value="id">Minion ID</option>
										<option value="os_type">OS Type</option>
										<option value="last_seen">Last seen</option>
										<option value="conformity_success">
											Conformity Success
										</option>
										<option value="conformity_incorrect">
											Conformity Incorrect
										</option>
										<option value="conformity_error">Conformity Error</option>
									</Select>
								)}
								{(f.fieldType === FilterFieldType.GRAIN ||
									f.fieldType === FilterFieldType.PACKAGE) && (
									<Input
										className="fl-span-3"
										onChange={(_e, data) =>
											updateFilter(f, 'field', data.value)
										}
										value={f.field}
										placeholder={
											f.fieldType === FilterFieldType.GRAIN
												? 'Grain name'
												: 'Package name'
										}
									/>
								)}

								{f.fieldType !== FilterFieldType.NONE && (
									<Select
										className="fl-span-2"
										onChange={(_e, data) =>
											updateFilter(f, 'operand', data.value)
										}
										value={f.operand}
									>
										{!(
											f.fieldType === FilterFieldType.OBJECT &&
											[
												'last_seen',
												'conformity_success',
												'conformity_incorrect',
												'conformity_error',
											].includes(f.field)
										) && (
											<>
												<option value={FilterOperand.CONTAINS}>
													Contains
												</option>
												<option value={FilterOperand.NOT_CONTAINS}>
													Not contains
												</option>
											</>
										)}
										<option value={FilterOperand.EQUALS}>Equals</option>
										<option value={FilterOperand.NOT_EQUALS}>Not equals</option>
										{!(
											f.fieldType === FilterFieldType.OBJECT &&
											[
												'last_seen',
												'conformity_success',
												'conformity_incorrect',
												'conformity_error',
											].includes(f.field)
										) && (
											<>
												<option value={FilterOperand.STARTS_WITH}>
													Starts with
												</option>
												<option value={FilterOperand.ENDS_WITH}>
													Ends with
												</option>
											</>
										)}
										<option value={FilterOperand.GREATER_THAN_OR_EQUAL}>
											&gt;=
										</option>
										<option value={FilterOperand.LESS_THAN_OR_EQUAL}>
											&lt;=
										</option>
									</Select>
								)}

								{f.fieldType !== FilterFieldType.NONE && (
									<Input
										className="fl-span-4"
										onChange={(_e, data) =>
											updateFilter(f, 'value', data.value)
										}
										value={f.value}
										placeholder="Value"
									/>
								)}
							</div>
						))}
						{filtersExpanded.value && (
							<CardFooter>
								<Button icon={<AddIcon />} onClick={addFilter}>
									Add Filter
								</Button>
							</CardFooter>
						)}
					</Card>
					<br />
					<Card>
						<CardHeader
							header={<span className={globalStyles.cardHeaderTitle}>Minions</span>}
						/>

						<Table sortable aria-label="Table with sort">
							<TableHeader>
								<TableRow>
									<TableHeaderCell {...headerSortProps('id')}>ID</TableHeaderCell>
									<TableHeaderCell {...headerSortProps('osType')}>
										OS
									</TableHeaderCell>
									<TableHeaderCell {...headerSortProps('lastUpdated')}>
										Last seen
									</TableHeaderCell>
									<TableHeaderCell {...headerSortProps('conformity')}>
										Conformity
									</TableHeaderCell>
								</TableRow>
							</TableHeader>
							<TableBody>
								{rows.map(({ item }) =>
									item.id.startsWith(SKEL) ? (
										<tr key={item.id}>
											<td className="p-m">
												<SkeletonItem size={16} />
											</td>
											<td className="p-m">
												<SkeletonItem size={16} />
											</td>
											<td className="p-m">
												<SkeletonItem size={16} />
											</td>
											<td className="p-m">
												<SkeletonItem size={16} />
											</td>
										</tr>
									) : (
										<TableRow key={item.id}>
											<TableCell>{item.id}</TableCell>
											<TableCell>{item.osType ?? 'Unknown'}</TableCell>
											<TableCell>{formatDate(item.lastSeen)}</TableCell>
											<TableCell>
												{item.lastUpdatedConformity === null ? (
													<Badge
														color="important"
														size="large"
														shape="rounded"
													>
														Unknown
													</Badge>
												) : (
													<>
														<Badge
															color="success"
															size="large"
															shape="rounded"
														>
															{item.conformitySuccess}
														</Badge>
														{' / '}
														<Badge
															color="warning"
															size="large"
															shape="rounded"
														>
															{item.conformityIncorrect}
														</Badge>
														{' / '}
														<Badge
															color="danger"
															size="large"
															shape="rounded"
														>
															{item.conformityError}
														</Badge>
													</>
												)}
											</TableCell>
											<TableCell>Actions</TableCell>
										</TableRow>
									),
								)}
							</TableBody>
						</Table>

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
