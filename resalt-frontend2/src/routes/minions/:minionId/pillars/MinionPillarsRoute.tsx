import { Card, CardHeader, SkeletonItem } from '@fluentui/react-components';
import { useEffect, useState } from 'react';
import { ObjectView } from 'react-object-view';
import { useParams } from 'react-router-dom';
import { getMinionById } from '../../../../lib/api';
import { ToastController } from '../../../../lib/toast';
import { jsonPalette, useGlobalStyles } from '../../../../lib/ui';
import MinionHeader from '../MinionHeader';

export default function MinionPillarsRoute(props: { toastController: ToastController }) {
	const { toastController } = props;
	const [loadingError, setLoadingError] = useState<Error | undefined>(undefined);
	const [pillars, setPillars] = useState<any | null | undefined>(undefined);
	const minionId = useParams().minionId!;
	const globalStyles = useGlobalStyles();

	useEffect(() => {
		// Fetch minion
		const abort = new AbortController();
		getMinionById(minionId, abort.signal)
			.then((minion) => {
				if (minion.pillars === null) {
					setPillars(null);
				} else {
					setPillars(JSON.parse(minion.pillars));
				}
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
			<MinionHeader tab="pillars" minionId={minionId!} error={loadingError} />
			<div className="fl-grid">
				<div className="fl-span-12">
					<Card style={{ height: '100%' }}>
						<CardHeader
							header={<span className={globalStyles.cardHeaderTitle}>Pillars</span>}
						/>
						{pillars === undefined ? (
							<SkeletonItem />
						) : pillars === null ? (
							<i>Unknown</i>
						) : (
							<ObjectView
								data={pillars}
								options={{
									hideDataTypes: true,
								}}
								styles={{}}
								palette={jsonPalette}
							/>
						)}
					</Card>
				</div>
			</div>
		</>
	);
}
