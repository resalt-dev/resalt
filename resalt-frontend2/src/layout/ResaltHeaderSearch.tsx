import { tokens } from '@fluentui/tokens';
import { ArrowRightRegular, DismissRegular, SearchRegular } from '@fluentui/react-icons';
import { Button, Input, makeStyles, mergeClasses, shorthands } from '@fluentui/react-components';
import { Signal, signal, useComputed } from '@preact/signals-react';

const styles = makeStyles({
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

const searchFocused = signal(false);
const searchValue: Signal<string> = signal('');

export default function ResaltHeaderSearch() {
	console.log('render:ResaltHeaderSearch');
	const classes = styles();

	function searchSubmit(e: React.FormEvent<HTMLFormElement>) {
		console.log('Search submitted: ' + searchValue.value);
		e.preventDefault();
	}

	// searchFocused.value || searchValue.value.length > 0
	const hasTextOrActive = useComputed(() => {
		return searchFocused.value || searchValue.value.length > 0;
	});
	const searchFieldClasses = mergeClasses(
		classes.headerSearchField,
		hasTextOrActive.value ? classes.headerSearchFieldActive : '',
	);
	const searchButtonClasses = mergeClasses(
		classes.headerSearchButton,
		hasTextOrActive.value ? classes.headerSearchButtonActive : '',
	);
	return (
		<form className={classes.headerSearchGrid} onSubmit={(e) => searchSubmit(e)}>
			<Input
				className={searchFieldClasses}
				size="medium"
				placeholder="Search Minion"
				onFocus={() => {
					searchFocused.value = true;
				}}
				onBlur={() => {
					searchFocused.value = false;
				}}
				value={searchValue.value}
				onChange={(_e, data) => {
					searchValue.value = data.value;
				}}
				contentBefore={
					false && hasTextOrActive.value ? (
						<></>
					) : (
						<SearchRegular className={classes.headerSearchButtonIcon} />
					)
				}
				contentAfter={
					searchValue.value.length > 0 ? (
						<DismissRegular
							className={classes.headerSearchDismissButton}
							onClick={() => {
								searchValue.value = '';
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
