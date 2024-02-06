import { makeStyles, mergeClasses, shorthands, tokens } from '@fluentui/react-components';
import { CaretDown16Filled, CaretRight16Filled, CaretUp16Filled } from '@fluentui/react-icons';
import { ConformityTreeNode, conformityMapFluentColor } from './ConformityTypes';

function calculateFullNamespace(node: ConformityTreeNode): string {
	// Traverse parents up
	let namespace = node.name;
	let parent = node.parent;
	while (parent && parent.name != '#') {
		namespace = parent.name + '.' + namespace;
		parent = parent.parent;
	}
	return namespace;
}

export const useStyles = makeStyles({
	box: {
		display: 'flex',
		alignItems: 'center',
	},
	icon: {
		display: 'inline',
		marginRight: tokens.spacingHorizontalS,
	},
	label: {
		display: 'inline',
	},
	labelHighlight: {
		color: tokens.colorPaletteDarkOrangeBorderActive,
		fontWeight: 'bold',
	},
	nodeCountLabel: {
		color: tokens.colorNeutralForeground4,
		paddingLeft: tokens.spacingHorizontalXS,
	},
	ul: {
		...shorthands.margin('0'),
		paddingLeft: tokens.spacingHorizontalS,
		listStyleType: 'none',
	},
	li: {
		position: 'relative',
		paddingTop: tokens.spacingVerticalSNudge,
		paddingBottom: tokens.spacingVerticalXS,
		paddingLeft: tokens.spacingVerticalL,
		boxSizing: 'border-box',
		'&:before': {
			position: 'absolute',
			top: tokens.spacingHorizontalL, // MUST be same as '&:last-child:after'.height
			left: 0,
			width: tokens.spacingHorizontalMNudge,
			height: '1px',
			...shorthands.margin('auto'),
			content: '""',
			backgroundColor: tokens.colorNeutralForeground2,
		},
		'&:after': {
			position: 'absolute',
			top: 0,
			bottom: 0,
			left: 0,
			width: '1px',
			height: '100%',
			content: '""',
			backgroundColor: tokens.colorNeutralForeground2,
		},
		'&:last-child:after': {
			height: tokens.spacingHorizontalL, // MUST be same as '&:before'.top
		},
	},
});

const UpIcon = CaretUp16Filled;
const DownIcon = CaretDown16Filled;
const RightIcon = CaretRight16Filled;

export function MinionConformityTree(props: {
	node: ConformityTreeNode;
	depth: number;
	filterNamespace: string;
	setFilterNamespace: React.Dispatch<React.SetStateAction<string>>;
	collapsedList: string[];
	setCollapsedList: React.Dispatch<React.SetStateAction<string[]>>;
}): JSX.Element {
	const { node, depth, filterNamespace, setFilterNamespace, collapsedList, setCollapsedList } =
		props;
	const sls = calculateFullNamespace(node);
	const color = conformityMapFluentColor(node.status);
	const collapsed = collapsedList.includes(sls);
	const Icon = collapsed ? UpIcon : sls === filterNamespace ? RightIcon : DownIcon;
	const styles = useStyles();

	return (
		<div>
			<div className={styles.box}>
				<Icon
					className={mergeClasses(styles.icon, node.name !== '#' ? 'mouse-pointer' : '')}
					style={{ color: color, border: `2px solid ${color}` }}
					onClick={() => {
						if (node.name === '#') return;
						if (collapsed) {
							setCollapsedList(collapsedList.filter((x) => x !== sls));
						} else {
							setCollapsedList([...collapsedList, sls]);
						}
					}}
				/>
				<span
					className="mouse-pointer"
					onClick={() => {
						if (sls === filterNamespace) {
							setFilterNamespace('');
						} else {
							setFilterNamespace(sls);
						}
					}}
				>
					<span
						className={mergeClasses(
							styles.label,
							sls === filterNamespace && styles.labelHighlight,
						)}
					>
						{node.name === '#' ? 'top.sls' : node.name}
					</span>
					<em className={styles.nodeCountLabel}>
						{node.items.length > 0
							? `(${node.items.length}${collapsed ? '+...' : ''})`
							: collapsed
								? '(...)'
								: null}
					</em>
				</span>
			</div>
			{collapsed ? null : (
				<ul className={styles.ul}>
					{node.subtree.map((subNode) => (
						<li key={subNode.name} className={styles.li}>
							<MinionConformityTree
								node={subNode}
								depth={depth + 1}
								filterNamespace={filterNamespace}
								setFilterNamespace={setFilterNamespace}
								collapsedList={collapsedList}
								setCollapsedList={setCollapsedList}
							/>
						</li>
					))}
				</ul>
			)}
		</div>
	);
}
