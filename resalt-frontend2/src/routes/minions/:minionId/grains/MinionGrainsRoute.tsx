import { Card, CardHeader, SkeletonItem } from '@fluentui/react-components';
import { useEffect, useState } from 'react';
import { ObjectView } from 'react-object-view';
import { useParams } from 'react-router-dom';
import { CopyButton } from '../../../../components/CopyButton';
import { getMinionById } from '../../../../lib/api';
import { ToastController } from '../../../../lib/toast';
import { jsonPalette } from '../../../../lib/ui';
import MinionHeader from '../MinionHeader';

export default function MinionGrainsRoute(props: { toastController: ToastController }) {
	const { toastController } = props;
	const [loadingError, setLoadingError] = useState<Error | undefined>(undefined);
	const [grains, setGrains] = useState<any | null | undefined>(undefined);
	const minionId = useParams().minionId!;

	useEffect(() => {
		// Fetch minion
		const abort = new AbortController();
		getMinionById(minionId, abort.signal)
			.then((minion) => {
				if (minion.grains === null) {
					setGrains(null);
				} else {
					setGrains(JSON.parse(minion.grains));
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
			<MinionHeader tab="grains" minionId={minionId!} error={loadingError} />
			<div className="fl-grid">
				<div className="fl-span-12">
					<Card style={{ height: '100%' }}>
						<CardHeader
							header={
								<>
									Grains
									<CopyButton
										name="grains"
										value={JSON.stringify(grains, null, '\t')}
										toastController={toastController}
									/>
								</>
							}
						/>
						{grains === undefined ? (
							<SkeletonItem />
						) : grains === null ? (
							<i>Unknown</i>
						) : (
							<ObjectView
								data={grains}
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
