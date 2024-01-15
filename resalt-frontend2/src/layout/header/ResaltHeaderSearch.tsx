import { Button, Input, makeStyles, mergeClasses, shorthands } from '@fluentui/react-components';
import { ArrowRightRegular, DismissRegular, SearchRegular } from '@fluentui/react-icons';
import { tokens } from '@fluentui/tokens';
import { useState } from 'react';

const useStyles = makeStyles({
	headerSearchGrid: {
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'center',
	},
	headerSearchField: {
		width: '80%',
		...shorthands.borderColor(tokens.colorNeutralStroke1),
		...shorthands.transition('width', tokens.durationNormal, tokens.curveEasyEase),
	},
	headerSearchFieldActive: {
		width: '100%',
	},
	headerSearchButton: {
		width: '32px',
		borderTopLeftRadius: '0',
		borderBottomLeftRadius: '0',
		// move it to the left
		marginLeft: '-12px',
		// move it higher in the z-index
		zIndex: 10,
		...shorthands.transition('opacity', tokens.durationNormal, tokens.curveEasyEase),
		opacity: '0',
	},
	headerSearchButtonActive: {
		opacity: '1',
	},
	headerSearchButtonIcon: {
		...shorthands.margin('0', tokens.spacingHorizontalS, '0', '0'),
	},
	headerSearchDismissButton: {
		cursor: 'pointer',
		...shorthands.margin('0', tokens.spacingHorizontalS, '0', '0'),
	},
});

export default function ResaltHeaderSearch() {
	const [searchFocused, setSearchFocused] = useState(false);
	const [searchValue, setSearchValue] = useState('');
	const styles = useStyles();

	function searchSubmit(e: React.FormEvent<HTMLFormElement>) {
		console.log('Search submitted: ' + searchValue);
		e.preventDefault();
	}

	const hasTextOrActive = searchFocused || searchValue.length > 0;
	const searchFieldClasses = mergeClasses(
		styles.headerSearchField,
		hasTextOrActive ? styles.headerSearchFieldActive : '',
	);
	const searchButtonClasses = mergeClasses(
		styles.headerSearchButton,
		hasTextOrActive ? styles.headerSearchButtonActive : '',
	);
	return (
		<form
			className={styles.headerSearchGrid}
			onSubmit={(e) => {
				searchSubmit(e);
			}}
		>
			<Input
				className={searchFieldClasses}
				size="medium"
				placeholder="Search Minion"
				onFocus={() => {
					setSearchFocused(true);
				}}
				onBlur={() => {
					setSearchFocused(false);
				}}
				value={searchValue}
				onChange={(_e, data) => {
					setSearchValue(data.value);
				}}
				contentBefore={<SearchRegular className={styles.headerSearchButtonIcon} />}
				contentAfter={
					searchValue.length > 0 ? (
						<DismissRegular
							className={styles.headerSearchDismissButton}
							onClick={() => {
								setSearchValue('');
							}}
						/>
					) : (
						<></>
					)
				}
			/>
			<Button type="submit" className={searchButtonClasses} icon={<ArrowRightRegular />} />
		</form>
	);
}
