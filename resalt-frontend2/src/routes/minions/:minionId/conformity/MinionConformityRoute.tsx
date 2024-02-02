import {
	Body1Stronger,
	Card,
	CardHeader,
	Checkbox,
	Radio,
	RadioGroup,
	SkeletonItem,
	tokens,
} from '@fluentui/react-components';
import { useEffect, useState } from 'react';
import { useParams, useSearchParams } from 'react-router-dom';
import { TerminalCard } from '../../../../components/TerminalCard';
import { getMinionById } from '../../../../lib/api';
import { ToastController } from '../../../../lib/toast';
import Minion from '../../../../models/Minion';
import MinionHeader from '../MinionHeader';
import { Conformity, ConformitySortOrder, parseConformity } from './ConformityTypes';

// Recursive function to render conformity changes
// for example output:
//     ----------
//     sudoku:
//         ----------
//         new:
//             1.0.5-2build3
//         old:
function MinionConformityChanges(data: unknown, pad: number): string {
	// If data is object, then print out "----------" and then the object
	if (typeof data === 'object') {
		return (
			' '.repeat(pad) +
			'----------\n' +
			Object.entries(data as object)
				.map(([key, value]) => {
					return `${' '.repeat(pad)}${key}:\n${MinionConformityChanges(value, pad + 4)}`;
				})
				.join('\n')
		);
	}
	// If data is not object, then print out the data
	return `${' '.repeat(pad)}${data}`;
}

export default function MinionConformityRoute(props: { toastController: ToastController }) {
	const { toastController } = props;
	const [searchParams, setSearchParams] = useSearchParams();
	const [loadingError, setLoadingError] = useState<Error | undefined>(undefined);
	const [minion, setMinion] = useState<Minion | null>(null);
	const [conformity, setConformity] = useState<Conformity[]>([]);
	const [sortOrder, setSortOrder] = useState<ConformitySortOrder>(
		Object.values(ConformitySortOrder).includes(searchParams.get('sort') as ConformitySortOrder)
			? (searchParams.get('sort') as ConformitySortOrder)
			: ConformitySortOrder.Incremental,
	);
	const [showSuccess, setShowSuccess] = useState<boolean>(searchParams.get('success') !== '0');
	const [showIncorrect, setShowIncorrect] = useState<boolean>(
		searchParams.get('incorrect') !== '0',
	);
	const [showError, setShowError] = useState<boolean>(searchParams.get('error') !== '0');
	const [showCollapsed, setShowCollapsed] = useState<boolean>(
		searchParams.get('collapsed') !== '0',
	);
	const [filterNamespace, setFilterNamespace] = useState<string>(
		searchParams.get('namespace') || '',
	);

	const minionId = useParams().minionId!;

	//
	// Settings
	//

	// Save settings to URL
	useEffect(() => {
		setSearchParams((search) => {
			sortOrder !== ConformitySortOrder.Incremental
				? search.set('sort', sortOrder)
				: search.delete('sort');
			!showSuccess ? search.set('success', '0') : search.delete('success');
			!showIncorrect ? search.set('incorrect', '0') : search.delete('incorrect');
			!showError ? search.set('error', '0') : search.delete('error');
			!showCollapsed ? search.set('collapsed', '0') : search.delete('collapsed');
			filterNamespace ? search.set('namespace', filterNamespace) : search.delete('namespace');
			return search;
		});
	}, [
		setSearchParams,
		sortOrder,
		showSuccess,
		showIncorrect,
		showError,
		showCollapsed,
		filterNamespace,
	]);

	//
	// Minion
	//

	useEffect(() => {
		// Fetch minion
		const abort = new AbortController();
		getMinionById(minionId, abort.signal)
			.then((minion: Minion) => {
				setMinion(minion);
				setConformity(parseConformity(minion?.conformity, sortOrder));
			})
			.catch((err: Error) => {
				setLoadingError(err);
				toastController.error('Error loading minion', err);
			});
		return () => {
			abort.abort();
		};
	}, [minionId, toastController]);

	return (
		<>
			<MinionHeader tab="conformity" minionId={minionId!} error={loadingError} />
			<div className="fl-grid">
				<div className="fl-span-3">
					<Card>
						<CardHeader header="Options" />
						<div>
							<Body1Stronger>Sort order</Body1Stronger>
							<br />
							<RadioGroup
								value={sortOrder}
								onChange={(_e, value) =>
									setSortOrder(value.value as ConformitySortOrder)
								}
							>
								<Radio
									value={ConformitySortOrder.Incremental}
									label="Incremental order"
								/>
								<Radio
									value={ConformitySortOrder.Decremental}
									label="Decremental order"
								/>
								<Radio
									value={ConformitySortOrder.LongestRuntime}
									label="Longest runtime"
								/>
								<Radio value={ConformitySortOrder.BestResult} label="Best result" />
								<Radio
									value={ConformitySortOrder.WorstResult}
									label="Errors first"
								/>
							</RadioGroup>
						</div>
						<div>
							<Body1Stronger>Visibility</Body1Stronger>
							<br />
							<Checkbox
								checked={showSuccess}
								onChange={() => setShowSuccess(!showSuccess)}
								label="Show succeeded"
								className="checkbox-success"
							/>
							<br />
							<Checkbox
								checked={showIncorrect}
								onChange={() => setShowIncorrect(!showIncorrect)}
								label="Show incorrect"
								className="checkbox-yellow"
							/>
							<br />
							<Checkbox
								checked={showError}
								onChange={() => setShowError(!showError)}
								label="Show errors"
								className="checkbox-danger"
							/>
							<br />
							<br />
							<Checkbox
								checked={showCollapsed}
								onChange={() => setShowCollapsed(!showCollapsed)}
								label="Show collapsed"
							/>
						</div>
					</Card>
					<br />
					<Card>
						<CardHeader header="States" />
						Options box
					</Card>
				</div>
				<div className="fl-span-9">
					{!minion ? (
						<SkeletonItem />
					) : (
						conformity.map((c) => {
							console.log(c);
							const color =
								c.status === 'success'
									? tokens.colorStatusSuccessBackground3
									: c.status === 'incorrect'
										? tokens.colorPaletteYellowBackground3
										: c.status === 'error'
											? tokens.colorStatusDangerBackground3
											: tokens.colorPalettePurpleBorderActive;
							const pad = 15;
							const title = c.data.__sls__ + ' // ' + c.data.__id__;
							return (
								<TerminalCard
									key={c.title}
									title={title}
									style={{ borderLeft: `5px solid ${color}`, rowGap: '0' }}
								>
									<div
										style={{
											color: color,
											lineHeight: '1.5',
											lineHeightStep: '0',
										}}
									>
										<pre>
											{'ID: '.padStart(pad)}
											{c.data.__id__}
											<br />
											{'Function: '.padStart(pad)}
											{c.fun}
											<br />
											{'Name: '.padStart(pad)}
											{c.data.name}
											<br />
											{'Result: '.padStart(pad)}
											{c.data.result == true
												? 'True'
												: c.data.result == false
													? 'False'
													: 'None'}
											<br />
											{'Comment: '.padStart(pad)}
											{c.data.comment
												.split('\n')
												.map((line: string) => {
													return line + '\n' + ' '.repeat(pad);
												})
												.join('')
												.trim()}
											<br />
											{'Started: '.padStart(pad)}
											{c.data.start_time}
											<br />
											{'Duration: '.padStart(pad)}
											{c.data.duration}
											<br />
											{'Changes: '.padStart(pad)}
											{MinionConformityChanges(c.data.changes, pad).trim()}
										</pre>
									</div>
								</TerminalCard>
							);
						})
					)}
				</div>
			</div>
		</>
	);
}
